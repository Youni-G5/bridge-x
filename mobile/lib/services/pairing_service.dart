import 'package:flutter/foundation.dart';
import 'bridge_service.dart';
import '../models/device.dart';

/// Service for handling device pairing
class PairingService extends ChangeNotifier {
  final BridgeService _bridgeService;

  PairingService(this._bridgeService);

  /// Process pairing QR code
  ///
  /// Format: bridgex://pair?id={device_id}&key={public_key_base64}
  Future<bool> processPairingQR(String qrData) async {
    try {
      debugPrint('Processing QR code: $qrData');

      // Parse URI
      final uri = Uri.parse(qrData);

      // Validate scheme
      if (uri.scheme != 'bridgex') {
        debugPrint('Invalid scheme: ${uri.scheme}');
        return false;
      }

      // Validate host
      if (uri.host != 'pair') {
        debugPrint('Invalid host: ${uri.host}');
        return false;
      }

      // Extract parameters
      final deviceId = uri.queryParameters['id'];
      final publicKeyBase64 = uri.queryParameters['key'];

      if (deviceId == null || publicKeyBase64 == null) {
        debugPrint('Missing parameters in QR code');
        return false;
      }

      debugPrint('Pairing with device: $deviceId');

      // TODO: Generate our own keypair and exchange keys
      // For now, just save the device
      final device = Device(
        id: deviceId,
        name: 'Desktop', // Could be extracted from QR or asked to user
        type: 'desktop',
        publicKey: publicKeyBase64,
        pairedAt: DateTime.now(),
      );

      _bridgeService.addPairedDevice(device);

      debugPrint('Device paired successfully');
      return true;
    } catch (e) {
      debugPrint('Error processing QR code: $e');
      return false;
    }
  }

  /// Generate pairing request (for desktop scanning mobile QR)
  Future<Map<String, dynamic>?> generatePairingQR(String deviceName) async {
    return await _bridgeService.requestPairing(deviceName);
  }
}
