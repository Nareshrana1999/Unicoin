import { useState, useEffect } from 'react';

export interface WalletInfo {
  id: string;
  name: string;
  address: string;
  balance: number;
  isConnected: boolean;
  network: string;
}

export interface Transaction {
  id: string;
  hash: string;
  type: 'sent' | 'received';
  amount: number;
  fee: number;
  timestamp: number;
  confirmations: number;
  status: 'pending' | 'confirmed' | 'failed';
  from: string;
  to: string;
}

export const useWallet = () => {
  const [wallet, setWallet] = useState<WalletInfo | null>(null);
  const [balance, setBalance] = useState(0);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    // Simulate wallet connection and data loading
    const loadWalletData = async () => {
      setIsLoading(true);
      
      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Mock wallet data
      const mockWallet: WalletInfo = {
        id: '1',
        name: 'My Unicoin Wallet',
        address: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
        balance: 1256.7890,
        isConnected: true,
        network: 'mainnet',
      };

      // Mock transactions
      const mockTransactions: Transaction[] = [
        {
          id: '1',
          hash: 'abc123def456...',
          type: 'received',
          amount: 5.2500,
          fee: 0.001,
          timestamp: Date.now() - 120000, // 2 minutes ago
          confirmations: 6,
          status: 'confirmed',
          from: 'UNI1abc123...',
          to: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
        },
        {
          id: '2',
          hash: 'def456ghi789...',
          type: 'sent',
          amount: 2.1000,
          fee: 0.001,
          timestamp: Date.now() - 3600000, // 1 hour ago
          confirmations: 12,
          status: 'confirmed',
          from: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
          to: 'UNI1def456...',
        },
        {
          id: '3',
          hash: 'ghi789jkl012...',
          type: 'received',
          amount: 10.0000,
          fee: 0.001,
          timestamp: Date.now() - 10800000, // 3 hours ago
          confirmations: 0,
          status: 'pending',
          from: 'UNI1ghi789...',
          to: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
        },
      ];

      setWallet(mockWallet);
      setBalance(mockWallet.balance);
      setIsConnected(mockWallet.isConnected);
      setTransactions(mockTransactions);
      setIsLoading(false);
    };

    loadWalletData();
  }, []);

  const connectWallet = async () => {
    setIsLoading(true);
    
    // Simulate wallet connection
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    const newWallet: WalletInfo = {
      id: '1',
      name: 'Connected Wallet',
      address: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
      balance: 1256.7890,
      isConnected: true,
      network: 'mainnet',
    };

    setWallet(newWallet);
    setBalance(newWallet.balance);
    setIsConnected(true);
    setIsLoading(false);
  };

  const disconnectWallet = () => {
    setWallet(null);
    setBalance(0);
    setIsConnected(false);
    setTransactions([]);
  };

  const sendTransaction = async (to: string, amount: number, fee: number = 0.001) => {
    if (!wallet || balance < amount + fee) {
      throw new Error('Insufficient balance');
    }

    setIsLoading(true);

    try {
      // Simulate transaction sending
      await new Promise(resolve => setTimeout(resolve, 2000));

      const newTransaction: Transaction = {
        id: Date.now().toString(),
        hash: `tx_${Math.random().toString(36).substr(2, 9)}...`,
        type: 'sent',
        amount,
        fee,
        timestamp: Date.now(),
        confirmations: 0,
        status: 'pending',
        from: wallet.address,
        to,
      };

      setTransactions(prev => [newTransaction, ...prev]);
      setBalance(prev => prev - amount - fee);

      return newTransaction;
    } finally {
      setIsLoading(false);
    }
  };

  const generateAddress = async () => {
    if (!wallet) {
      throw new Error('Wallet not connected');
    }

    // Simulate address generation
    await new Promise(resolve => setTimeout(resolve, 500));
    
    return `UNI1${Math.random().toString(36).substr(2, 42)}`;
  };

  const refreshBalance = async () => {
    if (!wallet) return;

    setIsLoading(true);
    
    // Simulate balance refresh
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Mock balance update
    const newBalance = balance + Math.random() * 10 - 5; // Random change
    setBalance(Math.max(0, newBalance));
    
    setIsLoading(false);
  };

  return {
    wallet,
    balance,
    isConnected,
    isLoading,
    transactions,
    connectWallet,
    disconnectWallet,
    sendTransaction,
    generateAddress,
    refreshBalance,
  };
};
