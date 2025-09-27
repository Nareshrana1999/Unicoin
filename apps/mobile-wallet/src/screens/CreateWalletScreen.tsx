import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  Alert,
  KeyboardAvoidingView,
  Platform,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import * as Animatable from 'react-native-animatable';
import { useWalletStore } from '../store/walletStore';
import { theme } from '../styles/theme';

export default function CreateWalletScreen() {
  const [step, setStep] = useState(1);
  const [walletName, setWalletName] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [seedPhrase, setSeedPhrase] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const { createWallet } = useWalletStore();

  const generateSeedPhrase = () => {
    // In a real implementation, this would generate a proper BIP39 seed phrase
    const words = [
      'abandon', 'abandon', 'abandon', 'abandon', 'abandon', 'abandon',
      'abandon', 'abandon', 'abandon', 'abandon', 'abandon', 'about'
    ];
    return words.join(' ');
  };

  const handleCreateWallet = async () => {
    if (step === 1) {
      // Validate form
      if (!walletName.trim()) {
        Alert.alert('Error', 'Please enter a wallet name');
        return;
      }
      if (!password.trim()) {
        Alert.alert('Error', 'Please enter a password');
        return;
      }
      if (password.length < 8) {
        Alert.alert('Error', 'Password must be at least 8 characters long');
        return;
      }
      if (password !== confirmPassword) {
        Alert.alert('Error', 'Passwords do not match');
        return;
      }

      // Generate seed phrase and move to next step
      const generatedSeedPhrase = generateSeedPhrase();
      setSeedPhrase(generatedSeedPhrase);
      setStep(2);
    } else if (step === 2) {
      // Create wallet
      setIsLoading(true);
      try {
        await createWallet(walletName, password);
        // Navigation will be handled by the store state change
      } catch (error) {
        Alert.alert('Error', 'Failed to create wallet. Please try again.');
      } finally {
        setIsLoading(false);
      }
    }
  };

  const renderStep1 = () => (
    <Animatable.View animation="fadeInUp" style={styles.stepContainer}>
      <View style={styles.iconContainer}>
        <LinearGradient
          colors={theme.gradients.primary}
          style={styles.iconGradient}
        >
          <Ionicons name="wallet" size={48} color={theme.colors.background} />
        </LinearGradient>
      </View>

      <Text style={styles.title}>Create New Wallet</Text>
      <Text style={styles.subtitle}>
        Set up your Unicoin wallet to start managing your cryptocurrency
      </Text>

      <View style={styles.formContainer}>
        <View style={styles.inputContainer}>
          <Text style={styles.inputLabel}>Wallet Name</Text>
          <TextInput
            style={styles.input}
            placeholder="Enter wallet name"
            value={walletName}
            onChangeText={setWalletName}
            autoCapitalize="none"
            autoCorrect={false}
          />
        </View>

        <View style={styles.inputContainer}>
          <Text style={styles.inputLabel}>Password</Text>
          <View style={styles.passwordContainer}>
            <TextInput
              style={styles.passwordInput}
              placeholder="Enter password"
              value={password}
              onChangeText={setPassword}
              secureTextEntry={!showPassword}
              autoCapitalize="none"
              autoCorrect={false}
            />
            <TouchableOpacity
              style={styles.passwordToggle}
              onPress={() => setShowPassword(!showPassword)}
            >
              <Ionicons
                name={showPassword ? 'eye-off' : 'eye'}
                size={20}
                color={theme.colors.textSecondary}
              />
            </TouchableOpacity>
          </View>
        </View>

        <View style={styles.inputContainer}>
          <Text style={styles.inputLabel}>Confirm Password</Text>
          <View style={styles.passwordContainer}>
            <TextInput
              style={styles.passwordInput}
              placeholder="Confirm password"
              value={confirmPassword}
              onChangeText={setConfirmPassword}
              secureTextEntry={!showConfirmPassword}
              autoCapitalize="none"
              autoCorrect={false}
            />
            <TouchableOpacity
              style={styles.passwordToggle}
              onPress={() => setShowConfirmPassword(!showConfirmPassword)}
            >
              <Ionicons
                name={showConfirmPassword ? 'eye-off' : 'eye'}
                size={20}
                color={theme.colors.textSecondary}
              />
            </TouchableOpacity>
          </View>
        </View>
      </View>
    </Animatable.View>
  );

  const renderStep2 = () => (
    <Animatable.View animation="fadeInUp" style={styles.stepContainer}>
      <View style={styles.iconContainer}>
        <LinearGradient
          colors={theme.gradients.warning}
          style={styles.iconGradient}
        >
          <Ionicons name="shield-checkmark" size={48} color={theme.colors.background} />
        </LinearGradient>
      </View>

      <Text style={styles.title}>Backup Your Wallet</Text>
      <Text style={styles.subtitle}>
        Save your seed phrase in a safe place. You'll need it to restore your wallet.
      </Text>

      <View style={styles.seedPhraseContainer}>
        <Text style={styles.seedPhraseLabel}>Your Seed Phrase</Text>
        <View style={styles.seedPhraseBox}>
          <Text style={styles.seedPhraseText}>{seedPhrase}</Text>
        </View>
        
        <TouchableOpacity
          style={styles.copyButton}
          onPress={() => {
            // In a real app, you would copy to clipboard
            Alert.alert('Copied', 'Seed phrase copied to clipboard');
          }}
        >
          <Ionicons name="copy" size={16} color={theme.colors.primary} />
          <Text style={styles.copyButtonText}>Copy to Clipboard</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.warningContainer}>
        <Ionicons name="warning" size={20} color={theme.colors.warning} />
        <Text style={styles.warningText}>
          Never share your seed phrase with anyone. Anyone with access to it can control your wallet.
        </Text>
      </View>
    </Animatable.View>
  );

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <ScrollView style={styles.scrollContainer}>
        <View style={styles.header}>
          <View style={styles.progressContainer}>
            <View style={styles.progressBar}>
              <View style={[styles.progressFill, { width: `${(step / 2) * 100}%` }]} />
            </View>
            <Text style={styles.progressText}>Step {step} of 2</Text>
          </View>
        </View>

        {step === 1 ? renderStep1() : renderStep2()}

        <View style={styles.buttonContainer}>
          <TouchableOpacity
            style={[
              styles.button,
              isLoading && styles.buttonDisabled,
            ]}
            onPress={handleCreateWallet}
            disabled={isLoading}
          >
            <LinearGradient
              colors={theme.gradients.primary}
              style={styles.buttonGradient}
            >
              <Text style={styles.buttonText}>
                {isLoading ? 'Creating...' : step === 1 ? 'Continue' : 'Create Wallet'}
              </Text>
            </LinearGradient>
          </TouchableOpacity>

          <TouchableOpacity
            style={styles.backButton}
            onPress={() => setStep(1)}
            disabled={step === 1}
          >
            <Text style={styles.backButtonText}>Back</Text>
          </TouchableOpacity>
        </View>
      </ScrollView>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: theme.colors.background,
  },
  scrollContainer: {
    flex: 1,
  },
  header: {
    padding: theme.spacing.lg,
    paddingTop: theme.spacing.xxl,
  },
  progressContainer: {
    alignItems: 'center',
  },
  progressBar: {
    width: '100%',
    height: 4,
    backgroundColor: theme.colors.border,
    borderRadius: 2,
    marginBottom: theme.spacing.sm,
  },
  progressFill: {
    height: '100%',
    backgroundColor: theme.colors.primary,
    borderRadius: 2,
  },
  progressText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
  },
  stepContainer: {
    padding: theme.spacing.lg,
    alignItems: 'center',
  },
  iconContainer: {
    marginBottom: theme.spacing.lg,
  },
  iconGradient: {
    width: 96,
    height: 96,
    borderRadius: 48,
    justifyContent: 'center',
    alignItems: 'center',
  },
  title: {
    fontSize: theme.fontSize.xxl,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
    textAlign: 'center',
    marginBottom: theme.spacing.sm,
  },
  subtitle: {
    fontSize: theme.fontSize.md,
    color: theme.colors.textSecondary,
    textAlign: 'center',
    lineHeight: 22,
    marginBottom: theme.spacing.xl,
  },
  formContainer: {
    width: '100%',
  },
  inputContainer: {
    marginBottom: theme.spacing.lg,
  },
  inputLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
    marginBottom: theme.spacing.sm,
  },
  input: {
    borderWidth: 1,
    borderColor: theme.colors.border,
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.md,
    fontSize: theme.fontSize.md,
    color: theme.colors.text,
    backgroundColor: theme.colors.surface,
  },
  passwordContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.border,
    borderRadius: theme.borderRadius.lg,
    backgroundColor: theme.colors.surface,
  },
  passwordInput: {
    flex: 1,
    padding: theme.spacing.md,
    fontSize: theme.fontSize.md,
    color: theme.colors.text,
  },
  passwordToggle: {
    padding: theme.spacing.md,
  },
  seedPhraseContainer: {
    width: '100%',
    marginBottom: theme.spacing.lg,
  },
  seedPhraseLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
    marginBottom: theme.spacing.sm,
  },
  seedPhraseBox: {
    backgroundColor: theme.colors.surface,
    borderWidth: 1,
    borderColor: theme.colors.border,
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.lg,
    marginBottom: theme.spacing.md,
  },
  seedPhraseText: {
    fontSize: theme.fontSize.md,
    color: theme.colors.text,
    lineHeight: 24,
    textAlign: 'center',
    fontWeight: theme.fontWeight.medium,
  },
  copyButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
  },
  copyButtonText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.primary,
    fontWeight: theme.fontWeight.medium,
    marginLeft: theme.spacing.xs,
  },
  warningContainer: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    backgroundColor: theme.colors.warning + '10',
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.md,
  },
  warningText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    lineHeight: 20,
    marginLeft: theme.spacing.sm,
    flex: 1,
  },
  buttonContainer: {
    padding: theme.spacing.lg,
  },
  button: {
    marginBottom: theme.spacing.md,
  },
  buttonDisabled: {
    opacity: 0.6,
  },
  buttonGradient: {
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.lg,
    alignItems: 'center',
  },
  buttonText: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.background,
    fontWeight: theme.fontWeight.bold,
  },
  backButton: {
    alignItems: 'center',
    padding: theme.spacing.md,
  },
  backButtonText: {
    fontSize: theme.fontSize.md,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
  },
});
