import 'dart:convert';
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ApiService {
  String _baseUrl = 'http://192.168.1.100:8080'; // Default, will be updated
  final _storage = const FlutterSecureStorage();
  String? _deviceId;
  String? _sessionToken;

  static const String _baseUrlKey = 'bridge_base_url';
  static const String _deviceIdKey = 'bridge_device_id';
  static const String _sessionTokenKey = 'bridge_session_token';

  ApiService() {
    _loadConfig();
  }

  Future<void> _loadConfig() async {
    _baseUrl = await _storage.read(key: _baseUrlKey) ?? _baseUrl;
    _deviceId = await _storage.read(key: _deviceIdKey);
    _sessionToken = await _storage.read(key: _sessionTokenKey);
  }

  Future<void> setBaseUrl(String url) async {
    _baseUrl = url;
    await _storage.write(key: _baseUrlKey, value: url);
  }

  String get baseUrl => _baseUrl;
  String? get deviceId => _deviceId;
  bool get isPaired => _deviceId != null && _sessionToken != null;

  // Health check
  Future<bool> checkHealth() async {
    try {
      final response = await http
          .get(Uri.parse('$_baseUrl/api/v1/health'))
          .timeout(const Duration(seconds: 5));
      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  // Pair device using QR code data
  Future<void> pairDevice({
    required String deviceName,
    required String pairingData,
  }) async {
    try {
      // Decode pairing data (contains server URL + public key)
      final decodedData = utf8.decode(base64.decode(pairingData));
      final pairingInfo = jsonDecode(decodedData);

      // Extract server URL and update
      if (pairingInfo['server_url'] != null) {
        await setBaseUrl(pairingInfo['server_url']);
      }

      // Send pairing request
      final response = await http.post(
        Uri.parse('$_baseUrl/api/v1/pair'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'device_name': deviceName,
          'public_key': pairingInfo['public_key'],
          'platform': Platform.operatingSystem,
        }),
      ).timeout(const Duration(seconds: 10));

      if (response.statusCode != 200) {
        throw Exception('Pairing failed: ${response.statusCode}');
      }

      final data = jsonDecode(response.body);

      // Store device credentials
      _deviceId = data['device_id'];
      _sessionToken = data['session_token'];

      await _storage.write(key: _deviceIdKey, value: _deviceId);
      await _storage.write(key: _sessionTokenKey, value: _sessionToken);
    } catch (e) {
      throw Exception('Pairing error: $e');
    }
  }

  // Get paired devices list
  Future<List<Map<String, dynamic>>> getDevices() async {
    try {
      final response = await http.get(
        Uri.parse('$_baseUrl/api/v1/devices'),
        headers: _getAuthHeaders(),
      ).timeout(const Duration(seconds: 10));

      if (response.statusCode != 200) {
        throw Exception('Failed to get devices: ${response.statusCode}');
      }

      final data = jsonDecode(response.body);
      return List<Map<String, dynamic>>.from(data['devices'] ?? []);
    } catch (e) {
      throw Exception('Get devices error: $e');
    }
  }

  // Send file to device
  Future<void> sendFile({
    required String deviceId,
    required String filePath,
    Function(double)? onProgress,
  }) async {
    try {
      final file = File(filePath);
      if (!await file.exists()) {
        throw Exception('File not found');
      }

      final fileSize = await file.length();
      final fileName = file.path.split('/').last;

      // 1. Initialize transfer
      final initResponse = await http.post(
        Uri.parse('$_baseUrl/api/v1/transfer/init'),
        headers: _getAuthHeaders(),
        body: jsonEncode({
          'device_id': deviceId,
          'file_name': fileName,
          'file_size': fileSize,
        }),
      ).timeout(const Duration(seconds: 10));

      if (initResponse.statusCode != 200) {
        throw Exception('Transfer init failed: ${initResponse.statusCode}');
      }

      final initData = jsonDecode(initResponse.body);
      final transferId = initData['transfer_id'];

      // 2. Upload file in chunks
      const chunkSize = 1024 * 1024; // 1MB chunks
      final fileBytes = await file.readAsBytes();
      int offset = 0;

      while (offset < fileBytes.length) {
        final end = (offset + chunkSize < fileBytes.length)
            ? offset + chunkSize
            : fileBytes.length;
        final chunk = fileBytes.sublist(offset, end);

        final request = http.MultipartRequest(
          'POST',
          Uri.parse('$_baseUrl/api/v1/transfer/upload'),
        );

        request.headers.addAll(_getAuthHeaders());
        request.fields['transfer_id'] = transferId;
        request.fields['offset'] = offset.toString();
        request.files.add(
          http.MultipartFile.fromBytes('chunk', chunk, filename: 'chunk'),
        );

        final streamedResponse = await request.send()
            .timeout(const Duration(seconds: 30));
        final uploadResponse = await http.Response.fromStream(streamedResponse);

        if (uploadResponse.statusCode != 200) {
          throw Exception('Chunk upload failed at offset $offset');
        }

        offset = end;

        // Report progress
        if (onProgress != null) {
          onProgress(offset / fileBytes.length);
        }
      }

      // 3. Finalize transfer
      final finalizeResponse = await http.post(
        Uri.parse('$_baseUrl/api/v1/transfer/finalize'),
        headers: _getAuthHeaders(),
        body: jsonEncode({'transfer_id': transferId}),
      ).timeout(const Duration(seconds: 10));

      if (finalizeResponse.statusCode != 200) {
        throw Exception('Transfer finalize failed: ${finalizeResponse.statusCode}');
      }
    } catch (e) {
      throw Exception('File transfer error: $e');
    }
  }

  // Delete paired device
  Future<void> unpairDevice(String targetDeviceId) async {
    try {
      final response = await http.delete(
        Uri.parse('$_baseUrl/api/v1/devices/$targetDeviceId'),
        headers: _getAuthHeaders(),
      ).timeout(const Duration(seconds: 10));

      if (response.statusCode != 200) {
        throw Exception('Unpair failed: ${response.statusCode}');
      }
    } catch (e) {
      throw Exception('Unpair error: $e');
    }
  }

  // Clear local pairing data (logout)
  Future<void> clearPairingData() async {
    await _storage.delete(key: _deviceIdKey);
    await _storage.delete(key: _sessionTokenKey);
    _deviceId = null;
    _sessionToken = null;
  }

  // Get auth headers
  Map<String, String> _getAuthHeaders() {
    final headers = {'Content-Type': 'application/json'};
    if (_sessionToken != null) {
      headers['Authorization'] = 'Bearer $_sessionToken';
    }
    return headers;
  }

  // Discover devices on local network
  Future<List<String>> discoverDevices() async {
    final discovered = <String>[];
    
    // Try common local IPs
    final baseIp = await _getLocalNetworkBase();
    
    for (int i = 1; i < 255; i++) {
      final ip = '$baseIp.$i';
      final testUrl = 'http://$ip:8080';
      
      try {
        final tempService = ApiService();
        await tempService.setBaseUrl(testUrl);
        if (await tempService.checkHealth()) {
          discovered.add(testUrl);
        }
      } catch (_) {
        // Ignore errors, continue scanning
      }
    }
    
    return discovered;
  }

  Future<String> _getLocalNetworkBase() async {
    try {
      // Get device's local IP to determine network base
      final interfaces = await NetworkInterface.list();
      for (var interface in interfaces) {
        for (var addr in interface.addresses) {
          if (addr.type == InternetAddressType.IPv4 && !addr.isLoopback) {
            final parts = addr.address.split('.');
            if (parts.length == 4) {
              return '${parts[0]}.${parts[1]}.${parts[2]}';
            }
          }
        }
      }
    } catch (_) {}
    
    return '192.168.1'; // Default fallback
  }
}
