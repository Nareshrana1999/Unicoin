import React, { useState, useEffect } from 'react';
import { View, Text, TouchableOpacity, Alert, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import * as LocalAuthentication from 'expo-local-authentication';

interface BiometricAuthProps {
  onSuccess: () => void;
  onFailure: () => void;
  onFallback: () => void;
}

const BiometricAuth: React.FC<BiometricAuthProps> = ({
  onSuccess,
  onFailure,
  onFallback,
}) => {
  const [biometryType, setBiometryType] = useState<LocalAuthentication.AuthenticationType | null>(null);
  const [isAvailable, setIsAvailable] = useState(false);
  const [isAuthenticating, setIsAuthenticating] = useState(false);

  useEffect(() => {
    checkBiometricAvailability();
  }, []);

  const checkBiometricAvailability = async () => {
    try {
      const compatible = await LocalAuthentication.hasHardwareAsync();
      const enrolled = await LocalAuthentication.isEnrolledAsync();
      const types = await LocalAuthentication.supportedAuthenticationTypesAsync();

      setIsAvailable(compatible && enrolled);
      setBiometryType(types[0] || null);
    } catch (error) {
      console.error('Error checking biometric availability:', error);
      setIsAvailable(false);
    }
  };

  const authenticateWithBiometrics = async () => {
    if (!isAvailable) {
      onFallback();
      return;
    }

    setIsAuthenticating(true);

    try {
      const result = await LocalAuthentication.authenticateAsync({
        promptMessage: 'Authenticate to access your wallet',
        cancelLabel: 'Cancel',
        fallbackLabel: 'Use Password',
        disableDeviceFallback: false,
      });

      if (result.success) {
        onSuccess();
      } else {
        onFailure();
      }
    } catch (error) {
      console.error('Biometric authentication error:', error);
      onFailure();
    } finally {
      setIsAuthenticating(false);
    }
  };

  const getBiometricIcon = () => {
    switch (biometryType) {
      case LocalAuthentication.AuthenticationType.FACIAL_RECOGNITION:
        return 'face-recognition';
      case LocalAuthentication.AuthenticationType.FINGERPRINT:
        return 'finger-print';
      case LocalAuthentication.AuthenticationType.IRIS:
        return 'eye';
      default:
        return 'shield-checkmark';
    }
  };

  const getBiometricText = () => {
    switch (biometryType) {
      case LocalAuthentication.AuthenticationType.FACIAL_RECOGNITION:
        return 'Face ID';
      case LocalAuthentication.AuthenticationType.FINGERPRINT:
        return 'Touch ID';
      case LocalAuthentication.AuthenticationType.IRIS:
        return 'Iris Scan';
      default:
        return 'Biometric';
    }
  };

  if (!isAvailable) {
    return null;
  }

  return (
    <View style={styles.container}>
      <TouchableOpacity
        style={[styles.biometricButton, isAuthenticating && styles.biometricButtonDisabled]}
        onPress={authenticateWithBiometrics}
        disabled={isAuthenticating}
      >
        <Ionicons
          name={getBiometricIcon() as any}
          size={32}
          color={isAuthenticating ? '#9CA3AF' : '#3B82F6'}
        />
        <Text style={[styles.biometricText, isAuthenticating && styles.biometricTextDisabled]}>
          {isAuthenticating ? 'Authenticating...' : `Use ${getBiometricText()}`}
        </Text>
      </TouchableOpacity>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginVertical: 20,
  },
  biometricButton: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#F3F4F6',
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderRadius: 12,
    borderWidth: 1,
    borderColor: '#E5E7EB',
  },
  biometricButtonDisabled: {
    backgroundColor: '#F9FAFB',
    borderColor: '#D1D5DB',
  },
  biometricText: {
    marginLeft: 8,
    fontSize: 16,
    fontWeight: '600',
    color: '#3B82F6',
  },
  biometricTextDisabled: {
    color: '#9CA3AF',
  },
});

export default BiometricAuth;
