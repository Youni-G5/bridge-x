import 'dart:io';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:crypto/crypto.dart';

/// Service for handling file transfers
class TransferService extends ChangeNotifier {
  static const String _defaultApiUrl = 'http://192.168.1.100:8080';
  String _apiUrl = _defaultApiUrl;

  void setApiUrl(String url) {
    _apiUrl = url;
    notifyListeners();
  }

  /// Upload a file to a device
  Future<void> uploadFile({
    required String deviceId,
    required String filePath,
    Function(double)? onProgress,
  }) async {
    final file = File(filePath);
    if (!file.existsSync()) {
      throw Exception('File not found: $filePath');
    }

    // Get file info
    final fileName = file.path.split('/').last;
    final fileSize = await file.length();

    // Calculate file hash
    debugPrint('Calculating hash for $fileName...');
    final fileHash = await _calculateFileHash(file);

    // Initialize transfer
    debugPrint('Initializing transfer...');
    final transferData = await _initializeTransfer(
      deviceId: deviceId,
      fileName: fileName,
      fileSize: fileSize,
      fileHash: fileHash,
    );

    if (transferData == null) {
      throw Exception('Failed to initialize transfer');
    }

    final transferId = transferData['transfer_id'] as String;
    debugPrint('Transfer ID: $transferId');

    // Upload file in chunks
    debugPrint('Uploading file...');
    await _uploadFileChunked(
      transferId: transferId,
      file: file,
      onProgress: onProgress,
    );

    debugPrint('Upload complete!');
  }

  /// Calculate SHA-256 hash of file
  Future<String> _calculateFileHash(File file) async {
    final bytes = await file.readAsBytes();
    final digest = sha256.convert(bytes);
    return 'sha256:${digest.toString()}';
  }

  /// Initialize transfer session
  Future<Map<String, dynamic>?> _initializeTransfer({
    required String deviceId,
    required String fileName,
    required int fileSize,
    required String fileHash,
  }) async {
    try {
      final response = await http.post(
        Uri.parse('$_apiUrl/api/v1/transfer'),
        headers: {'Content-Type': 'application/json'},
        body: json.encode({
          'device_id': deviceId,
          'file_name': fileName,
          'file_size': fileSize,
          'file_hash': fileHash,
        }),
      );

      if (response.statusCode == 200) {
        return json.decode(response.body) as Map<String, dynamic>;
      } else {
        debugPrint('Transfer init failed: ${response.statusCode}');
        return null;
      }
    } catch (e) {
      debugPrint('Transfer init error: $e');
      return null;
    }
  }

  /// Upload file in chunks
  Future<void> _uploadFileChunked({
    required String transferId,
    required File file,
    Function(double)? onProgress,
  }) async {
    const chunkSize = 1024 * 1024; // 1MB chunks
    final fileSize = await file.length();
    final totalChunks = (fileSize / chunkSize).ceil();

    final fileStream = file.openRead();
    var chunkIndex = 0;
    var bytesUploaded = 0;

    await for (var chunk in fileStream) {
      // Upload chunk
      final response = await http.post(
        Uri.parse('$_apiUrl/api/v1/transfer/$transferId/upload'),
        headers: {
          'Content-Type': 'application/octet-stream',
          'X-Chunk-Index': chunkIndex.toString(),
          'X-Total-Chunks': totalChunks.toString(),
        },
        body: chunk,
      );

      if (response.statusCode != 200) {
        throw Exception('Chunk upload failed: ${response.statusCode}');
      }

      // Update progress
      bytesUploaded += chunk.length;
      final progress = bytesUploaded / fileSize;
      onProgress?.call(progress);

      chunkIndex++;
    }
  }
}
