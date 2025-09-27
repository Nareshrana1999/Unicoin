import React, { useEffect } from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  RefreshControl,
  TouchableOpacity,
  Dimensions,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import * as Animatable from 'react-native-animatable';
import { useWalletStore } from '../store/walletStore';
import { theme } from '../styles/theme';

const { width } = Dimensions.get('window');

export default function HomeScreen() {
  const {
    currentWallet,
    transactions,
    isOnline,
    isSyncing,
    updateBalance,
    refreshTransactions,
    setOnlineStatus,
    setSyncingStatus,
  } = useWalletStore();

  const [refreshing, setRefreshing] = React.useState(false);

  useEffect(() => {
    // Initialize data
    updateBalance();
    refreshTransactions();
    setOnlineStatus(true);
  }, []);

  const onRefresh = async () => {
    setRefreshing(true);
    try {
      await Promise.all([
        updateBalance(),
        refreshTransactions(),
      ]);
    } finally {
      setRefreshing(false);
    }
  };

  const quickActions = [
    {
      title: 'Send',
      icon: 'send',
      color: theme.colors.primary,
      onPress: () => {
        // Navigate to send screen
      },
    },
    {
      title: 'Receive',
      icon: 'download',
      color: theme.colors.success,
      onPress: () => {
        // Navigate to receive screen
      },
    },
    {
      title: 'Swap',
      icon: 'swap-horizontal',
      color: theme.colors.secondary,
      onPress: () => {
        // Navigate to swap screen
      },
    },
    {
      title: 'Stake',
      icon: 'trending-up',
      color: theme.colors.warning,
      onPress: () => {
        // Navigate to stake screen
      },
    },
  ];

  const recentTransactions = transactions.slice(0, 3);

  return (
    <ScrollView
      style={styles.container}
      refreshControl={
        <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
      }
    >
      {/* Header */}
      <View style={styles.header}>
        <View style={styles.headerTop}>
          <View>
            <Text style={styles.greeting}>Good morning</Text>
            <Text style={styles.walletName}>{currentWallet?.name || 'Wallet'}</Text>
          </View>
          <View style={styles.statusContainer}>
            <View style={[styles.statusDot, { backgroundColor: isOnline ? theme.colors.success : theme.colors.error }]} />
            <Text style={styles.statusText}>{isOnline ? 'Online' : 'Offline'}</Text>
          </View>
        </View>

        {/* Balance Card */}
        <Animatable.View animation="fadeInUp" delay={200}>
          <LinearGradient
            colors={theme.gradients.primary}
            style={styles.balanceCard}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
          >
            <Text style={styles.balanceLabel}>Total Balance</Text>
            <Text style={styles.balanceAmount}>
              {currentWallet?.balance.toFixed(4) || '0.0000'} UNI
            </Text>
            <Text style={styles.balanceUSD}>
              ≈ ${((currentWallet?.balance || 0) * 0.2456).toFixed(2)} USD
            </Text>
            
            {isSyncing && (
              <View style={styles.syncingContainer}>
                <Ionicons name="sync" size={16} color={theme.colors.text} />
                <Text style={styles.syncingText}>Syncing...</Text>
              </View>
            )}
          </LinearGradient>
        </Animatable.View>
      </View>

      {/* Quick Actions */}
      <Animatable.View animation="fadeInUp" delay={400}>
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Quick Actions</Text>
          <View style={styles.quickActionsContainer}>
            {quickActions.map((action, index) => (
              <TouchableOpacity
                key={action.title}
                style={styles.quickAction}
                onPress={action.onPress}
              >
                <View style={[styles.quickActionIcon, { backgroundColor: action.color }]}>
                  <Ionicons name={action.icon as any} size={24} color={theme.colors.background} />
                </View>
                <Text style={styles.quickActionText}>{action.title}</Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>
      </Animatable.View>

      {/* Recent Transactions */}
      <Animatable.View animation="fadeInUp" delay={600}>
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Recent Transactions</Text>
            <TouchableOpacity>
              <Text style={styles.viewAllText}>View All</Text>
            </TouchableOpacity>
          </View>
          
          {recentTransactions.length > 0 ? (
            <View style={styles.transactionsContainer}>
              {recentTransactions.map((tx, index) => (
                <View key={tx.id} style={styles.transactionItem}>
                  <View style={styles.transactionIcon}>
                    <Ionicons
                      name={tx.type === 'sent' ? 'arrow-up' : 'arrow-down'}
                      size={20}
                      color={tx.type === 'sent' ? theme.colors.error : theme.colors.success}
                    />
                  </View>
                  <View style={styles.transactionDetails}>
                    <Text style={styles.transactionType}>
                      {tx.type === 'sent' ? 'Sent' : 'Received'}
                    </Text>
                    <Text style={styles.transactionHash}>
                      {tx.type === 'sent' ? `To: ${tx.to.slice(0, 8)}...` : `From: ${tx.from.slice(0, 8)}...`}
                    </Text>
                  </View>
                  <View style={styles.transactionAmount}>
                    <Text style={[
                      styles.transactionAmountText,
                      { color: tx.type === 'sent' ? theme.colors.error : theme.colors.success }
                    ]}>
                      {tx.type === 'sent' ? '-' : '+'}{tx.amount.toFixed(4)} UNI
                    </Text>
                    <Text style={styles.transactionStatus}>{tx.status}</Text>
                  </View>
                </View>
              ))}
            </View>
          ) : (
            <View style={styles.emptyState}>
              <Ionicons name="receipt-outline" size={48} color={theme.colors.gray} />
              <Text style={styles.emptyStateText}>No transactions yet</Text>
              <Text style={styles.emptyStateSubtext}>
                Your transaction history will appear here
              </Text>
            </View>
          )}
        </View>
      </Animatable.View>

      {/* Market Overview */}
      <Animatable.View animation="fadeInUp" delay={800}>
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Market Overview</Text>
          <View style={styles.marketContainer}>
            <View style={styles.marketItem}>
              <Text style={styles.marketLabel}>UNI Price</Text>
              <Text style={styles.marketValue}>$0.2456</Text>
              <Text style={[styles.marketChange, { color: theme.colors.success }]}>+5.2%</Text>
            </View>
            <View style={styles.marketItem}>
              <Text style={styles.marketLabel}>Market Cap</Text>
              <Text style={styles.marketValue}>$51.6M</Text>
              <Text style={[styles.marketChange, { color: theme.colors.success }]}>+8.1%</Text>
            </View>
            <View style={styles.marketItem}>
              <Text style={styles.marketLabel}>24h Volume</Text>
              <Text style={styles.marketValue}>210K</Text>
              <Text style={[styles.marketChange, { color: theme.colors.error }]}>-2.3%</Text>
            </View>
          </View>
        </View>
      </Animatable.View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: theme.colors.background,
  },
  header: {
    padding: theme.spacing.md,
    paddingTop: theme.spacing.lg,
  },
  headerTop: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  greeting: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
  },
  walletName: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
    marginTop: theme.spacing.xs,
  },
  statusContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    marginRight: theme.spacing.xs,
  },
  statusText: {
    fontSize: theme.fontSize.xs,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
  },
  balanceCard: {
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.xl,
    ...theme.shadows.lg,
  },
  balanceLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.background,
    opacity: 0.8,
    fontWeight: theme.fontWeight.medium,
  },
  balanceAmount: {
    fontSize: theme.fontSize.xxxl,
    color: theme.colors.background,
    fontWeight: theme.fontWeight.bold,
    marginTop: theme.spacing.xs,
  },
  balanceUSD: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.background,
    opacity: 0.8,
    fontWeight: theme.fontWeight.medium,
    marginTop: theme.spacing.xs,
  },
  syncingContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: theme.spacing.md,
  },
  syncingText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.background,
    opacity: 0.8,
    marginLeft: theme.spacing.xs,
  },
  section: {
    padding: theme.spacing.md,
  },
  sectionTitle: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
    marginBottom: theme.spacing.md,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.md,
  },
  viewAllText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.primary,
    fontWeight: theme.fontWeight.medium,
  },
  quickActionsContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  quickAction: {
    alignItems: 'center',
    flex: 1,
  },
  quickActionIcon: {
    width: 56,
    height: 56,
    borderRadius: 28,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  quickActionText: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
    textAlign: 'center',
  },
  transactionsContainer: {
    backgroundColor: theme.colors.card,
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.md,
    ...theme.shadows.sm,
  },
  transactionItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: theme.spacing.sm,
  },
  transactionIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: theme.colors.surface,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.md,
  },
  transactionDetails: {
    flex: 1,
  },
  transactionType: {
    fontSize: theme.fontSize.md,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.medium,
  },
  transactionHash: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    marginTop: theme.spacing.xs,
  },
  transactionAmount: {
    alignItems: 'flex-end',
  },
  transactionAmountText: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
  },
  transactionStatus: {
    fontSize: theme.fontSize.xs,
    color: theme.colors.textSecondary,
    marginTop: theme.spacing.xs,
  },
  emptyState: {
    alignItems: 'center',
    paddingVertical: theme.spacing.xxl,
  },
  emptyStateText: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
    marginTop: theme.spacing.md,
  },
  emptyStateSubtext: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textTertiary,
    marginTop: theme.spacing.xs,
    textAlign: 'center',
  },
  marketContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  marketItem: {
    flex: 1,
    alignItems: 'center',
    padding: theme.spacing.md,
    backgroundColor: theme.colors.card,
    borderRadius: theme.borderRadius.lg,
    marginHorizontal: theme.spacing.xs,
    ...theme.shadows.sm,
  },
  marketLabel: {
    fontSize: theme.fontSize.sm,
    color: theme.colors.textSecondary,
    fontWeight: theme.fontWeight.medium,
    marginBottom: theme.spacing.xs,
  },
  marketValue: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.text,
    fontWeight: theme.fontWeight.bold,
    marginBottom: theme.spacing.xs,
  },
  marketChange: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.medium,
  },
});
