import 'package:flutter/material.dart';
import 'package:qr_code_scanner/qr_code_scanner.dart';
import 'package:provider/provider.dart';
import '../services/pairing_service.dart';

class QRScannerScreen extends StatefulWidget {
  const QRScannerScreen({super.key});

  @override
  State<QRScannerScreen> createState() => _QRScannerScreenState();
}

class _QRScannerScreenState extends State<QRScannerScreen> {
  final GlobalKey qrKey = GlobalKey(debugLabel: 'QR');
  QRViewController? controller;
  bool _isProcessing = false;

  @override
  void dispose() {
    controller?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Scan QR Code'),
        backgroundColor: Colors.black,
      ),
      body: Column(
        children: [
          Expanded(
            flex: 5,
            child: QRView(
              key: qrKey,
              onQRViewCreated: _onQRViewCreated,
              overlay: QrScannerOverlayShape(
                borderColor: Colors.blue,
                borderRadius: 10,
                borderLength: 30,
                borderWidth: 10,
                cutOutSize: 300,
              ),
            ),
          ),
          Expanded(
            flex: 1,
            child: Container(
              color: Colors.black87,
              child: Center(
                child: _isProcessing
                    ? const Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          CircularProgressIndicator(color: Colors.white),
                          SizedBox(height: 16),
                          Text(
                            'Processing pairing...',
                            style: TextStyle(color: Colors.white),
                          ),
                        ],
                      )
                    : const Text(
                        'Scan the QR code from your desktop app',
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 16,
                        ),
                        textAlign: TextAlign.center,
                      ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _onQRViewCreated(QRViewController controller) {
    this.controller = controller;
    controller.scannedDataStream.listen((scanData) {
      if (!_isProcessing && scanData.code != null) {
        _handleQRCode(scanData.code!);
      }
    });
  }

  Future<void> _handleQRCode(String qrData) async {
    setState(() {
      _isProcessing = true;
    });

    try {
      // Pause scanning
      await controller?.pauseCamera();

      final pairingService = context.read<PairingService>();
      final success = await pairingService.processPairingQR(qrData);

      if (!mounted) return;

      if (success) {
        // Show success message
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('✅ Device paired successfully!'),
            backgroundColor: Colors.green,
          ),
        );

        // Go back to home
        Navigator.pop(context, true);
      } else {
        // Show error and resume scanning
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('❌ Pairing failed. Please try again.'),
            backgroundColor: Colors.red,
          ),
        );

        await controller?.resumeCamera();
        setState(() {
          _isProcessing = false;
        });
      }
    } catch (e) {
      if (!mounted) return;

      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Error: $e'),
          backgroundColor: Colors.red,
        ),
      );

      await controller?.resumeCamera();
      setState(() {
        _isProcessing = false;
      });
    }
  }
}
