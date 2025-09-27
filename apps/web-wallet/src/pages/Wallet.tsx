import React from 'react';
import { motion } from 'framer-motion';
import { useWallet } from '../hooks/useWallet';

const Wallet: React.FC = () => {
  const { wallet, balance, isConnected } = useWallet();

  return (
    <div className="max-w-6xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Wallet</h1>
        <p className="text-gray-600">
          Manage your addresses and wallet settings.
        </p>
      </div>

      {isConnected && wallet ? (
        <div className="space-y-8">
          {/* Wallet Overview */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
          >
            <h2 className="text-xl font-semibold text-gray-900 mb-6">Wallet Overview</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="text-center">
                <div className="text-3xl font-bold text-gray-900">{balance.toFixed(4)}</div>
                <div className="text-sm text-gray-600">Total Balance (UNI)</div>
              </div>
              <div className="text-center">
                <div className="text-3xl font-bold text-gray-900">12</div>
                <div className="text-sm text-gray-600">Total Addresses</div>
              </div>
              <div className="text-center">
                <div className="text-3xl font-bold text-gray-900">156</div>
                <div className="text-sm text-gray-600">Transactions</div>
              </div>
            </div>
          </motion.div>

          {/* Address Management */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
          >
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-xl font-semibold text-gray-900">Addresses</h2>
              <button className="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
                Generate New
              </button>
            </div>
            <div className="space-y-4">
              {/* Address list would go here */}
              <p className="text-gray-500 text-center py-8">Address management coming soon...</p>
            </div>
          </motion.div>
        </div>
      ) : (
        <div className="text-center py-12">
          <div className="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <span className="text-4xl">🔒</span>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Connect Your Wallet</h2>
          <p className="text-gray-600 mb-8">
            Please connect your wallet to manage addresses.
          </p>
          <button className="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
            Connect Wallet
          </button>
        </div>
      )}
    </div>
  );
};

export default Wallet;
