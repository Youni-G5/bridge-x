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
    checkHealth();
    loadDevices();

    // Event listeners
    refreshBtn.addEventListener('click', checkHealth);
    pairBtn.addEventListener('click', handlePairDevice);
    sendFileBtn.addEventListener('click', handleSendFile);
});

// Check backend health
async function checkHealth() {
    try {
        statusText.textContent = 'Checking...';
        statusElement.className = 'status-indicator';

        const result = await invoke('check_health');
        
        statusText.textContent = 'Connected';
        statusElement.className = 'status-indicator connected';
    } catch (error) {
        console.error('Health check failed:', error);
        statusText.textContent = 'Disconnected';
        statusElement.className = 'status-indicator error';
    }
}

// Load paired devices
async function loadDevices() {
    try {
        const devices = await invoke('get_devices');
        
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
                        <h4>💻 ${device}</h4>
                        <p>Last seen: Just now</p>
                    </div>
                    <button class="btn-icon" onclick="removeDevice('${device}')">
                        🗑️
                    </button>
                </div>
            `).join('');
        }
    } catch (error) {
        console.error('Failed to load devices:', error);
    }
}

// Handle device pairing
async function handlePairDevice() {
    try {
        const deviceName = 'Desktop PC';
        const result = await invoke('pair_device', { deviceName });
        
        // Show QR modal
        qrModal.classList.remove('hidden');
        
        // TODO: Display actual QR code
        document.getElementById('qr-code').innerHTML = `
            <div style="padding: 50px; text-align: center;">
                <p style="font-size: 3rem;">📱</p>
                <p>QR Code Placeholder</p>
                <small>Scan with mobile app</small>
            </div>
        `;
        
        console.log('Pairing:', result);
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
        // TODO: Open file picker
        alert('File picker coming soon...');
    } catch (error) {
        console.error('Send file failed:', error);
    }
}

// Remove device
function removeDevice(deviceName) {
    if (confirm(`Remove ${deviceName}?`)) {
        // TODO: Remove device via backend
        loadDevices();
    }
}

// Auto-refresh every 10 seconds
setInterval(checkHealth, 10000);
