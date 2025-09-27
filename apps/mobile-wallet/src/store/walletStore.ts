import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import AsyncStorage from '@react-native-async-storage/async-storage';
import * as SecureStore from 'expo-secure-store';

export interface WalletInfo {
  id: string;
  name: string;
  address: string;
  balance: number;
  isConnected: boolean;
  network: string;
  createdAt: string;
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
  memo?: string;
}

export interface AddressInfo {
  address: string;
  label?: string;
  balance: number;
  isActive: boolean;
}

interface WalletState {
  // Wallet state
  isWalletCreated: boolean;
  isUnlocked: boolean;
  currentWallet: WalletInfo | null;
  wallets: WalletInfo[];
  
  // Transaction state
  transactions: Transaction[];
  isLoadingTransactions: boolean;
  
  // Address state
  addresses: AddressInfo[];
  currentAddress: string;
  
  // Network state
  isOnline: boolean;
  isSyncing: boolean;
  
  // Actions
  createWallet: (name: string, password: string) => Promise<void>;
  importWallet: (seedPhrase: string, name: string, password: string) => Promise<void>;
  unlockWallet: (password: string) => Promise<boolean>;
  lockWallet: () => void;
  switchWallet: (walletId: string) => void;
  updateBalance: () => Promise<void>;
  sendTransaction: (to: string, amount: number, fee: number, memo?: string) => Promise<Transaction>;
  generateNewAddress: () => Promise<string>;
  refreshTransactions: () => Promise<void>;
  setOnlineStatus: (isOnline: boolean) => void;
  setSyncingStatus: (isSyncing: boolean) => void;
}

export const useWalletStore = create<WalletState>()(
  persist(
    (set, get) => ({
      // Initial state
      isWalletCreated: false,
      isUnlocked: false,
      currentWallet: null,
      wallets: [],
      transactions: [],
      isLoadingTransactions: false,
      addresses: [],
      currentAddress: '',
      isOnline: false,
      isSyncing: false,

      // Actions
      createWallet: async (name: string, password: string) => {
        try {
          // Generate new wallet
          const walletId = `wallet_${Date.now()}`;
          const address = `UNI1${Math.random().toString(36).substr(2, 42)}`;
          
          const newWallet: WalletInfo = {
            id: walletId,
            name,
            address,
            balance: 0,
            isConnected: true,
            network: 'mainnet',
            createdAt: new Date().toISOString(),
          };

          // Store wallet securely
          await SecureStore.setItemAsync(`wallet_${walletId}`, JSON.stringify(newWallet));
          await SecureStore.setItemAsync(`password_${walletId}`, password);

          // Generate initial address
          const initialAddress: AddressInfo = {
            address,
            label: 'Primary Address',
            balance: 0,
            isActive: true,
          };

          set({
            isWalletCreated: true,
            isUnlocked: true,
            currentWallet: newWallet,
            wallets: [newWallet],
            addresses: [initialAddress],
            currentAddress: address,
          });
        } catch (error) {
          console.error('Error creating wallet:', error);
          throw new Error('Failed to create wallet');
        }
      },

      importWallet: async (seedPhrase: string, name: string, password: string) => {
        try {
          // Validate seed phrase
          const words = seedPhrase.trim().split(' ');
          if (words.length !== 12 && words.length !== 24) {
            throw new Error('Invalid seed phrase length');
          }

          // Generate wallet from seed phrase
          const walletId = `wallet_${Date.now()}`;
          const address = `UNI1${Math.random().toString(36).substr(2, 42)}`;
          
          const newWallet: WalletInfo = {
            id: walletId,
            name,
            address,
            balance: 0,
            isConnected: true,
            network: 'mainnet',
            createdAt: new Date().toISOString(),
          };

          // Store wallet securely
          await SecureStore.setItemAsync(`wallet_${walletId}`, JSON.stringify(newWallet));
          await SecureStore.setItemAsync(`password_${walletId}`, password);
          await SecureStore.setItemAsync(`seed_${walletId}`, seedPhrase);

          // Generate initial address
          const initialAddress: AddressInfo = {
            address,
            label: 'Primary Address',
            balance: 0,
            isActive: true,
          };

          set({
            isWalletCreated: true,
            isUnlocked: true,
            currentWallet: newWallet,
            wallets: [newWallet],
            addresses: [initialAddress],
            currentAddress: address,
          });
        } catch (error) {
          console.error('Error importing wallet:', error);
          throw new Error('Failed to import wallet');
        }
      },

      unlockWallet: async (password: string) => {
        try {
          const { currentWallet } = get();
          if (!currentWallet) {
            throw new Error('No wallet found');
          }

          // Verify password
          const storedPassword = await SecureStore.getItemAsync(`password_${currentWallet.id}`);
          if (storedPassword !== password) {
            return false;
          }

          set({ isUnlocked: true });
          return true;
        } catch (error) {
          console.error('Error unlocking wallet:', error);
          return false;
        }
      },

      lockWallet: () => {
        set({ isUnlocked: false });
      },

      switchWallet: (walletId: string) => {
        const { wallets } = get();
        const wallet = wallets.find(w => w.id === walletId);
        if (wallet) {
          set({ 
            currentWallet: wallet,
            currentAddress: wallet.address,
            isUnlocked: false, // Lock when switching wallets
          });
        }
      },

      updateBalance: async () => {
        try {
          const { currentWallet } = get();
          if (!currentWallet) return;

          // Simulate API call to get balance
          await new Promise(resolve => setTimeout(resolve, 1000));
          
          const newBalance = Math.random() * 1000; // Mock balance

          set(state => ({
            currentWallet: state.currentWallet ? {
              ...state.currentWallet,
              balance: newBalance,
            } : null,
            addresses: state.addresses.map(addr => 
              addr.address === state.currentAddress 
                ? { ...addr, balance: newBalance }
                : addr
            ),
          }));
        } catch (error) {
          console.error('Error updating balance:', error);
        }
      },

      sendTransaction: async (to: string, amount: number, fee: number, memo?: string) => {
        try {
          const { currentWallet } = get();
          if (!currentWallet) {
            throw new Error('No wallet found');
          }

          if (currentWallet.balance < amount + fee) {
            throw new Error('Insufficient balance');
          }

          // Create transaction
          const transaction: Transaction = {
            id: `tx_${Date.now()}`,
            hash: `hash_${Math.random().toString(36).substr(2, 9)}`,
            type: 'sent',
            amount,
            fee,
            timestamp: Date.now(),
            confirmations: 0,
            status: 'pending',
            from: currentWallet.address,
            to,
            memo,
          };

          // Add to transactions
          set(state => ({
            transactions: [transaction, ...state.transactions],
            currentWallet: state.currentWallet ? {
              ...state.currentWallet,
              balance: state.currentWallet.balance - amount - fee,
            } : null,
          }));

          return transaction;
        } catch (error) {
          console.error('Error sending transaction:', error);
          throw error;
        }
      },

      generateNewAddress: async () => {
        try {
          const newAddress = `UNI1${Math.random().toString(36).substr(2, 42)}`;
          
          const newAddressInfo: AddressInfo = {
            address: newAddress,
            label: `Address ${get().addresses.length + 1}`,
            balance: 0,
            isActive: true,
          };

          set(state => ({
            addresses: [...state.addresses, newAddressInfo],
          }));

          return newAddress;
        } catch (error) {
          console.error('Error generating address:', error);
          throw new Error('Failed to generate address');
        }
      },

      refreshTransactions: async () => {
        try {
          set({ isLoadingTransactions: true });

          // Simulate API call
          await new Promise(resolve => setTimeout(resolve, 1000));

          // Mock transactions
          const mockTransactions: Transaction[] = [
            {
              id: '1',
              hash: 'abc123def456...',
              type: 'received',
              amount: 5.25,
              fee: 0.001,
              timestamp: Date.now() - 120000,
              confirmations: 6,
              status: 'confirmed',
              from: 'UNI1abc123...',
              to: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
            },
            {
              id: '2',
              hash: 'def456ghi789...',
              type: 'sent',
              amount: 2.1,
              fee: 0.001,
              timestamp: Date.now() - 3600000,
              confirmations: 12,
              status: 'confirmed',
              from: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
              to: 'UNI1def456...',
            },
            {
              id: '3',
              hash: 'ghi789jkl012...',
              type: 'received',
              amount: 10.0,
              fee: 0.001,
              timestamp: Date.now() - 10800000,
              confirmations: 0,
              status: 'pending',
              from: 'UNI1ghi789...',
              to: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
            },
          ];

          set({ 
            transactions: mockTransactions,
            isLoadingTransactions: false,
          });
        } catch (error) {
          console.error('Error refreshing transactions:', error);
          set({ isLoadingTransactions: false });
        }
      },

      setOnlineStatus: (isOnline: boolean) => {
        set({ isOnline });
      },

      setSyncingStatus: (isSyncing: boolean) => {
        set({ isSyncing });
      },
    }),
    {
      name: 'wallet-storage',
      storage: createJSONStorage(() => AsyncStorage),
      partialize: (state) => ({
        isWalletCreated: state.isWalletCreated,
        currentWallet: state.currentWallet,
        wallets: state.wallets,
        addresses: state.addresses,
        currentAddress: state.currentAddress,
      }),
    }
  )
);
