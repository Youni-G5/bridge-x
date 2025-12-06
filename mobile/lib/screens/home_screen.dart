import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'qr_scanner_screen.dart';
import 'devices_screen.dart';
import '../services/api_service.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final _apiService = ApiService();
  bool _isCheckingConnection = false;
  bool _isConnected = false;

  @override
  void initState() {
    super.initState();
    _checkConnection();
    _requestPermissions();
  }

  Future<void> _checkConnection() async {
    setState(() {
      _isCheckingConnection = true;
    });

    try {
      final isHealthy = await _apiService.checkHealth();
      setState(() {
        _isConnected = isHealthy;
      });
    } catch (e) {
      setState(() {
        _isConnected = false;
      });
    } finally {
      setState(() {
        _isCheckingConnection = false;
      });
    }
  }

  Future<void> _requestPermissions() async {
    await Permission.camera.request();
    await Permission.storage.request();
  }

  Future<void> _scanQR() async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => const QRScannerScreen()),
    );

    if (result == true && mounted) {
      // Pairing successful
      _checkConnection();
    }
  }

  void _openDevices() {
    Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => const DevicesScreen()),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('BridgeX'),
        centerTitle: true,
        actions: [
          if (_isCheckingConnection)
            const Center(
              child: Padding(
                padding: EdgeInsets.all(16.0),
                child: SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
              ),
            )
          else
            IconButton(
              icon: Icon(
                _isConnected ? Icons.check_circle : Icons.error,
                color: _isConnected ? Colors.green : Colors.red,
              ),
              onPressed: _checkConnection,
              tooltip: _isConnected ? 'Connecté' : 'Déconnecté',
            ),
        ],
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.sync_alt,
                size: 120,
                color: Theme.of(context).primaryColor.withOpacity(0.5),
              ),
              const SizedBox(height: 40),
              Text(
                'Transfert de fichiers P2P',
                style: Theme.of(context).textTheme.headlineSmall,
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
              Text(
                'Chiffré · Local · Sécurisé',
                style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                      color: Colors.grey,
                    ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 60),
              
              // Pair Device Button
              ElevatedButton.icon(
                onPressed: _scanQR,
                icon: const Icon(Icons.qr_code_scanner),
                label: const Text('Scanner QR Code'),
                style: ElevatedButton.styleFrom(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 32,
                    vertical: 16,
                  ),
                  textStyle: const TextStyle(fontSize: 18),
                ),
              ),
              
              const SizedBox(height: 16),
              
              // Devices Button
              if (_apiService.isPaired)
                OutlinedButton.icon(
                  onPressed: _openDevices,
                  icon: const Icon(Icons.devices),
                  label: const Text('Mes appareils'),
                  style: OutlinedButton.styleFrom(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 32,
                      vertical: 16,
                    ),
                    textStyle: const TextStyle(fontSize: 18),
                  ),
                ),

              const SizedBox(height: 40),

              // Connection Status
              if (!_isConnected && !_isCheckingConnection)
                Card(
                  color: Colors.orange.shade50,
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Row(
                      children: [
                        Icon(Icons.warning_amber, color: Colors.orange.shade700),
                        const SizedBox(width: 12),
                        Expanded(
                          child: Text(
                            'Aucune connexion au PC.\nAssurez-vous d\'\u00eatre sur le m\u00eame WiFi.',
                            style: TextStyle(color: Colors.orange.shade900),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
