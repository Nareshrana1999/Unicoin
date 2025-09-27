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

export default function SendScreen() {
  const [recipientAddress, setRecipientAddress] = useState('');
  const [amount, setAmount] = useState('');
  const [fee, setFee] = useState('0.001');
  const [memo, setMemo] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const { currentWallet, sendTransaction } = useWalletStore();

  const handleSend = async () => {
    if (!recipientAddress.trim()) {
      Alert.alert('Error', 'Please enter recipient address');
      return;
    }
    if (!amount.trim()) {
      Alert.alert('Error', 'Please enter amount');
      return;
    }

    const amountNum = parseFloat(amount);
    const feeNum = parseFloat(fee);

    if (isNaN(amountNum) || amountNum <= 0) {
      Alert.alert('Error', 'Please enter a valid amount');
      return;
    }

    if (currentWallet && amountNum + feeNum > currentWallet.balance) {
      Alert.alert('Error', 'Insufficient balance');
      return;
    }

    setIsLoading(true);
    try {
      await sendTransaction(recipientAddress, amountNum, feeNum, memo || undefined);
      Alert.alert('Success', 'Transaction sent successfully!', [
        { text: 'OK', onPress: () => {
          setRecipientAddress('');
          setAmount('');
          setMemo('');
        }}
      ]);
    } catch (error) {
      Alert.alert('Error', 'Failed to send transaction');
    } finally {
      setIsLoading(false);
    }
  };

  const quickAmounts = ['0.1', '0.5', '1.0', '5.0', '10.0'];

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <ScrollView style={styles.scrollContainer}>
        <Animatable.View animation="fadeInUp">
          <View style={styles.balanceCard}>
            <Text style={styles.balanceLabel}>Available Balance</Text>
            <Text style={styles.balanceAmount}>
              {currentWallet?.balance.toFixed(4) || '0.0000'} UNI
            </Text>
          </View>
        </Animatable.View>

        <Animatable.View animation="fadeInUp" delay={200}>
          <View style={styles.formContainer}>
            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Recipient Address</Text>
              <TextInput
                style={styles.input}
                placeholder="Enter UNI address"
                value={recipientAddress}
                onChangeText={setRecipientAddress}
                autoCapitalize="none"
                autoCorrect={false}
              />
              <TouchableOpacity style={styles.scanButton}>
                <Ionicons name="qr-code" size={20} color={theme.colors.primary} />
                <Text style={styles.scanButtonText}>Scan QR</Text>
              </TouchableOpacity>
            </View>

            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Amount (UNI)</Text>
              <TextInput
                style={styles.input}
                placeholder="0.0000"
                value={amount}
                onChangeText={setAmount}
                keyboardType="decimal-pad"
              />
              <TouchableOpacity
                style={styles.maxButton}
                onPress={() => {
                  if (currentWallet) {
                    setAmount((currentWallet.balance - parseFloat(fee)).toFixed(4));
                  }
                }}
              >
                <Text style={styles.maxButtonText}>MAX</Text>
              </TouchableOpacity>
            </View>

            <View style={styles.quickAmountsContainer}>
              <Text style={styles.quickAmountsLabel}>Quick Amounts</Text>
              <View style={styles.quickAmounts}>
                {quickAmounts.map((quickAmount) => (
                  <TouchableOpacity
                    key={quickAmount}
                    style={styles.quickAmountButton}
                    onPress={() => setAmount(quickAmount)}
                  >
                    <Text style={styles.quickAmountText}>{quickAmount} UNI</Text>
                  </TouchableOpacity>
                ))}
              </View>
            </View>

            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Transaction Fee (UNI)</Text>
              <TextInput
                style={styles.input}
                placeholder="0.001"
                value={fee}
                onChangeText={setFee}
                keyboardType="decimal-pad"
              />
            </View>

            <View style={styles.inputContainer}>
              <Text style={styles.inputLabel}>Memo (Optional)</Text>
              <TextInput
                style={[styles.input, styles.memoInput]}
                placeholder="Add a note..."
                value={memo}
                onChangeText={setMemo}
                multiline
                numberOfLines={3}
              />
            </View>
          </View>
        </Animatable.View>

        <Animatable.View animation="fadeInUp" delay={400}>
          <View style={styles.summaryContainer}>
            <Text style={styles.summaryTitle}>Transaction Summary</Text>
            <View style={styles.summaryItem}>
              <Text style={styles.summaryLabel}>Amount:</Text>
              <Text style={styles.summaryValue}>{amount || '0.0000'} UNI</Text>
            </View>
            <View style={styles.summaryItem}>
              <Text style={styles.summaryLabel}>Fee:</Text>
              <Text style={styles.summaryValue}>{fee} UNI</Text>
            </View>
            <View style={[styles.summaryItem, styles.summaryTotal]}>
              <Text style={styles.summaryLabel}>Total:</Text>
              <Text style={styles.summaryValue}>
                {(parseFloat(amount || '0') + parseFloat(fee)).toFixed(4)} UNI
              </Text>
            </View>
          </View>
        </Animatable.View>

        <View style={styles.buttonContainer}>
          <TouchableOpacity
            style={[styles.button, isLoading && styles.buttonDisabled]}
            onPress={handleSend}
            disabled={isLoading}
          >
            <LinearGradient
              colors={theme.gradients.primary}
              style={styles.buttonGradient}
            >
              <Text style={styles.buttonText}>
                {isLoading ? 'Sending...' : 'Send Transaction'}
              </Text>
            </LinearGradient>
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
    padding: theme.spacing.md,
  },
  balanceCard: {
    backgroundColor: theme.colors.card,
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.lg,
    marginBottom: theme.spacing.lg,
    alignItems: 'center',
    ...theme.shadows.md,
  },
  balanceLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
    marginBottom: theme.spacing.xs,
  },
  balanceAmount: {
    fontSize: theme.fontSize.xxl,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
  },
  formContainer: {
    marginBottom: theme.spacing.lg,
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
  memoInput: {
    height: 80,
    textAlignVertical: 'top',
  },
  scanButton: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: theme.spacing.sm,
  },
  scanButtonText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.primary,
    fontWeight: theme.fontWeight.medium,
    marginLeft: theme.spacing.xs,
  },
  maxButton: {
    position: 'absolute',
    right: theme.spacing.md,
    top: 32,
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: theme.spacing.xs,
    backgroundColor: theme.colors.primary,
    borderRadius: theme.borderRadius.sm,
  },
  maxButtonText: {
    fontSize: theme.fontSize.xs,
    color: theme.colors.background,
    fontWeight: theme.fontWeight.bold,
  },
  quickAmountsContainer: {
    marginBottom: theme.spacing.lg,
  },
  quickAmountsLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
    marginBottom: theme.spacing.sm,
  },
  quickAmounts: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: theme.spacing.sm,
  },
  quickAmountButton: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    backgroundColor: theme.colors.surface,
    borderRadius: theme.borderRadius.lg,
    borderWidth: 1,
    borderColor: theme.colors.border,
  },
  quickAmountText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
  },
  summaryContainer: {
    backgroundColor: theme.colors.card,
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.lg,
    marginBottom: theme.spacing.lg,
    ...theme.shadows.sm,
  },
  summaryTitle: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
    marginBottom: theme.spacing.md,
  },
  summaryItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: theme.spacing.sm,
  },
  summaryTotal: {
    borderTopWidth: 1,
    borderTopColor: theme.colors.border,
    marginTop: theme.spacing.sm,
    paddingTop: theme.spacing.md,
  },
  summaryLabel: {
    fontSize: theme.fontSize.md,
    color: theme.colors.textSecondary,
  },
  summaryValue: {
    fontSize: theme.fontSize.md,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
  },
  buttonContainer: {
    paddingBottom: theme.spacing.lg,
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
});
