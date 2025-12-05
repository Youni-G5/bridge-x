// Tauri API
const { invoke } = window.__TAURI__.core;

// Elements
const statusElement = document.getElementById('status');
const statusText = document.getElementById('status-text');
const refreshBtn = document.getElementById('refresh-btn');
const pairBtn = document.getElementById('pair-btn');
const sendFileBtn = document.getElementById('send-file-btn');
const devicesList = document.getElementById('devices-list');
const qrModal = document.getElementById('qr-modal');

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    // Wait a bit for backend to start
    setTimeout(() => {
        checkHealth();
        loadDevices();
    }, 2000);

    // Event listeners
    refreshBtn.addEventListener('click', () => {
        checkHealth();
        loadDevices();
    });
    pairBtn.addEventListener('click', handlePairDevice);
    sendFileBtn.addEventListener('click', handleSendFile);
});

// Check backend health
async function checkHealth() {
    try {
        statusText.textContent = 'Checking...';
        statusElement.className = 'status-indicator';

        const result = await invoke('check_health');
        const data = JSON.parse(result);
        
        statusText.textContent = `Connected (v${data.version})`;
        statusElement.className = 'status-indicator connected';
        
        // Enable buttons
        pairBtn.disabled = false;
    } catch (error) {
        console.error('Health check failed:', error);
        statusText.textContent = 'Disconnected';
        statusElement.className = 'status-indicator error';
        
        // Disable buttons
        pairBtn.disabled = true;
    }
}

// Load paired devices
async function loadDevices() {
    try {
        const result = await invoke('get_devices');
        const devices = JSON.parse(result);
        
        if (devices.length === 0) {
            devicesList.innerHTML = `
                <div class="empty-state">
                    <p>No paired devices</p>
                    <small>Pair a device to get started</small>
                </div>
            `;
        } else {
            devicesList.innerHTML = devices.map(device => `
                <div class="device-item">
                    <div class="device-info">
                        <h4>📱 ${device.name}</h4>
                        <p>Type: ${device.device_type || device.type}</p>
                        <small>ID: ${device.id.substring(0, 8)}...</small>
                    </div>
                    <button class="btn-icon" onclick="removeDevice('${device.id}')" title="Unpair">
                        🗑️
                    </button>
                </div>
            `).join('');
        }
    } catch (error) {
        console.error('Failed to load devices:', error);
        devicesList.innerHTML = `
            <div class="empty-state" style="color: #dc3545;">
                <p>Failed to load devices</p>
                <small>${error}</small>
            </div>
        `;
    }
}

// Handle device pairing
async function handlePairDevice() {
    try {
        const deviceName = 'Desktop PC';
        const result = await invoke('pair_device', { deviceName });
        const data = JSON.parse(result);
        
        console.log('Pairing response:', data);
        
        // Show QR modal with actual QR code
        qrModal.classList.remove('hidden');
        
        // Display the QR code image from backend
        if (data.qr_data) {
            document.getElementById('qr-code').innerHTML = `
                <img src="${data.qr_data}" alt="Pairing QR Code" style="width: 100%; height: 100%; object-fit: contain;" />
            `;
        } else {
            document.getElementById('qr-code').innerHTML = `
                <div style="padding: 50px; text-align: center;">
                    <p style="font-size: 3rem;">📱</p>
                    <p>QR Code</p>
                    <small>Device ID: ${data.device_id.substring(0, 8)}...</small>
                </div>
            `;
        }
        
        // Auto-reload devices after pairing
        setTimeout(() => {
            loadDevices();
        }, 1000);
        
    } catch (error) {
        console.error('Pairing failed:', error);
        alert('Pairing failed: ' + error);
    }
}

// Close QR modal
function closeQRModal() {
    qrModal.classList.add('hidden');
}

// Handle file sending
async function handleSendFile() {
    try {
        alert('File picker coming soon...\n\nThis will open a file dialog to select files for transfer.');
    } catch (error) {
        console.error('Send file failed:', error);
    }
}

// Remove device
async function removeDevice(deviceId) {
    if (confirm('Remove this device?')) {
        try {
            // TODO: Call backend delete endpoint
            console.log('Removing device:', deviceId);
            loadDevices();
        } catch (error) {
            console.error('Failed to remove device:', error);
        }
    }
}

// Make function available globally for onclick
window.removeDevice = removeDevice;
window.closeQRModal = closeQRModal;

// Auto-refresh every 30 seconds
setInterval(() => {
    checkHealth();
}, 30000);
