import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet, Alert, TouchableOpacity } from 'react-native';
import { RNCamera } from 'react-native-camera';
import QRCodeScanner from 'react-native-qrcode-scanner';
import { Ionicons } from '@expo/vector-icons';

interface QRCodeScannerProps {
  onScan: (data: string) => void;
  onClose: () => void;
  title?: string;
}

const QRCodeScannerComponent: React.FC<QRCodeScannerProps> = ({
  onScan,
  onClose,
  title = 'Scan QR Code',
}) => {
  const [hasPermission, setHasPermission] = useState<boolean | null>(null);
  const [scanned, setScanned] = useState(false);

  useEffect(() => {
    checkCameraPermission();
  }, []);

  const checkCameraPermission = async () => {
    try {
      const { status } = await RNCamera.requestCameraPermission();
      setHasPermission(status === 'authorized');
    } catch (error) {
      console.error('Error requesting camera permission:', error);
      setHasPermission(false);
    }
  };

  const handleScan = (e: any) => {
    if (scanned) return;
    
    setScanned(true);
    const data = e.data;
    
    // Validate QR code data
    if (isValidAddress(data) || isValidTransaction(data)) {
      onScan(data);
    } else {
      Alert.alert(
        'Invalid QR Code',
        'This QR code does not contain a valid Unicoin address or transaction.',
        [
          { text: 'Try Again', onPress: () => setScanned(false) },
          { text: 'Cancel', onPress: onClose },
        ]
      );
    }
  };

  const isValidAddress = (data: string): boolean => {
    // Check if it's a valid Unicoin address
    return data.startsWith('UNI1') && data.length >= 42;
  };

  const isValidTransaction = (data: string): boolean => {
    // Check if it's a valid transaction data
    try {
      const parsed = JSON.parse(data);
      return parsed.type === 'unicoin_transaction' && parsed.address;
    } catch {
      return false;
    }
  };

  if (hasPermission === null) {
    return (
      <View style={styles.container}>
        <Text style={styles.message}>Requesting camera permission...</Text>
      </View>
    );
  }

  if (hasPermission === false) {
    return (
      <View style={styles.container}>
        <Text style={styles.message}>Camera permission denied</Text>
        <TouchableOpacity style={styles.button} onPress={onClose}>
          <Text style={styles.buttonText}>Close</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>{title}</Text>
        <TouchableOpacity style={styles.closeButton} onPress={onClose}>
          <Ionicons name="close" size={24} color="#FFFFFF" />
        </TouchableOpacity>
      </View>
      
      <QRCodeScanner
        onRead={handleScan}
        flashMode={RNCamera.Constants.FlashMode.auto}
        topContent={
          <Text style={styles.instructions}>
            Position the QR code within the frame to scan
          </Text>
        }
        bottomContent={
          <View style={styles.bottomContent}>
            <TouchableOpacity style={styles.button} onPress={onClose}>
              <Text style={styles.buttonText}>Cancel</Text>
            </TouchableOpacity>
          </View>
        }
        cameraStyle={styles.camera}
        showMarker={true}
        markerStyle={styles.marker}
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000000',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingTop: 50,
    paddingBottom: 20,
    backgroundColor: 'rgba(0, 0, 0, 0.8)',
  },
  title: {
    fontSize: 18,
    fontWeight: '600',
    color: '#FFFFFF',
  },
  closeButton: {
    padding: 8,
  },
  instructions: {
    fontSize: 16,
    color: '#FFFFFF',
    textAlign: 'center',
    marginBottom: 20,
  },
  bottomContent: {
    padding: 20,
    backgroundColor: 'rgba(0, 0, 0, 0.8)',
  },
  button: {
    backgroundColor: '#3B82F6',
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderRadius: 8,
    alignItems: 'center',
  },
  buttonText: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
  camera: {
    flex: 1,
  },
  marker: {
    borderColor: '#3B82F6',
    borderWidth: 2,
  },
  message: {
    fontSize: 16,
    color: '#FFFFFF',
    textAlign: 'center',
    marginTop: 50,
  },
});

export default QRCodeScannerComponent;
