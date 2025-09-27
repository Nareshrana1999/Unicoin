import React from 'react';
import { motion } from 'framer-motion';
import { useWallet } from '../hooks/useWallet';

const History: React.FC = () => {
  const { transactions } = useWallet();

  return (
    <div className="max-w-6xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Transaction History</h1>
        <p className="text-gray-600">
          View all your Unicoin transactions.
        </p>
      </div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white rounded-xl shadow-lg border border-gray-200"
      >
        <div className="p-6 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-900">All Transactions</h2>
            <div className="flex space-x-2">
              <button className="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded-lg">
                All
              </button>
              <button className="px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 rounded-lg">
                Sent
              </button>
              <button className="px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 rounded-lg">
                Received
              </button>
            </div>
          </div>
        </div>

        <div className="divide-y divide-gray-200">
          {transactions.map((tx, index) => (
            <motion.div
              key={tx.id}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: index * 0.1 }}
              className="p-6 hover:bg-gray-50 transition-colors duration-200"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-4">
                  <div className={`w-10 h-10 rounded-full flex items-center justify-center ${
                    tx.type === 'received' 
                      ? 'bg-green-100 text-green-600' 
                      : 'bg-blue-100 text-blue-600'
                  }`}>
                    {tx.type === 'received' ? '📥' : '📤'}
                  </div>
                  <div>
                    <div className="font-semibold text-gray-900">
                      {tx.type === 'received' ? 'Received' : 'Sent'} {tx.amount.toFixed(4)} UNI
                    </div>
                    <div className="text-sm text-gray-600">
                      {tx.type === 'received' ? `From ${tx.from}` : `To ${tx.to}`}
                    </div>
                  </div>
                </div>
                
                <div className="text-right">
                  <div className="text-sm text-gray-500">
                    {new Date(tx.timestamp).toLocaleString()}
                  </div>
                  <div className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                    tx.status === 'confirmed' 
                      ? 'bg-green-100 text-green-800' 
                      : 'bg-yellow-100 text-yellow-800'
                  }`}>
                    {tx.status}
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </motion.div>
    </div>
  );
};

export default History;
