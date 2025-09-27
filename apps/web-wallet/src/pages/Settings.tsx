import React from 'react';
import { motion } from 'framer-motion';

const Settings: React.FC = () => {
  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Settings</h1>
        <p className="text-gray-600">
          Manage your wallet preferences and security settings.
        </p>
      </div>

      <div className="space-y-8">
        {/* General Settings */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
        >
          <h2 className="text-xl font-semibold text-gray-900 mb-6">General Settings</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Wallet Name
              </label>
              <input
                type="text"
                defaultValue="My Unicoin Wallet"
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Default Currency
              </label>
              <select className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200">
                <option value="UNI">UNI (Unicoin)</option>
                <option value="USD">USD</option>
                <option value="BTC">BTC</option>
                <option value="ETH">ETH</option>
              </select>
            </div>
          </div>
        </motion.div>

        {/* Security Settings */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
        >
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Security</h2>
          <div className="space-y-4">
            <button className="w-full text-left p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors duration-200">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-medium text-gray-900">Change Password</div>
                  <div className="text-sm text-gray-600">Update your wallet password</div>
                </div>
                <span className="text-gray-400">→</span>
              </div>
            </button>
            
            <button className="w-full text-left p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors duration-200">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-medium text-gray-900">Backup Wallet</div>
                  <div className="text-sm text-gray-600">Export your wallet data</div>
                </div>
                <span className="text-gray-400">→</span>
              </div>
            </button>
            
            <button className="w-full text-left p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors duration-200">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-medium text-gray-900">Lock Wallet</div>
                  <div className="text-sm text-gray-600">Secure your wallet with a timeout</div>
                </div>
                <span className="text-gray-400">→</span>
              </div>
            </button>
          </div>
        </motion.div>

        {/* Network Settings */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
        >
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Network</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Network
              </label>
              <select className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200">
                <option value="mainnet">Mainnet</option>
                <option value="testnet">Testnet</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Node Connection
              </label>
              <input
                type="text"
                defaultValue="https://api.unicoin.org"
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
              />
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  );
};

export default Settings;
