import React from 'react';
import { motion } from 'framer-motion';

const Explorer: React.FC = () => {
  return (
    <div className="max-w-7xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Block Explorer</h1>
        <p className="text-gray-600">
          Explore the Unicoin blockchain and search for transactions, blocks, and addresses.
        </p>
      </div>

      {/* Search Bar */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white rounded-xl shadow-lg border border-gray-200 p-6 mb-8"
      >
        <div className="flex space-x-4">
          <input
            type="text"
            placeholder="Search by address, transaction hash, or block height..."
            className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
          />
          <button className="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
            Search
          </button>
        </div>
      </motion.div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        {[
          { name: 'Latest Block', value: '1,234,567', change: '+12' },
          { name: 'Hash Rate', value: '125.6 TH/s', change: '+5.2%' },
          { name: 'Difficulty', value: '15.2T', change: '+3.1%' },
          { name: 'Network Size', value: '2,456 nodes', change: '+23' },
        ].map((stat, index) => (
          <motion.div
            key={stat.name}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            className="bg-white rounded-xl shadow-lg border border-gray-200 p-6"
          >
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900">{stat.value}</div>
              <div className="text-sm text-gray-600">{stat.name}</div>
              <div className="text-xs text-green-600 mt-1">{stat.change}</div>
            </div>
          </motion.div>
        ))}
      </div>

      {/* Recent Blocks and Transactions */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Recent Blocks */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200"
        >
          <div className="p-6 border-b border-gray-200">
            <h2 className="text-xl font-semibold text-gray-900">Recent Blocks</h2>
          </div>
          <div className="divide-y divide-gray-200">
            {[1, 2, 3, 4, 5].map((block) => (
              <div key={block} className="p-6 hover:bg-gray-50 transition-colors duration-200">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="font-semibold text-gray-900">Block #{1234567 - block}</div>
                    <div className="text-sm text-gray-600">
                      {new Date(Date.now() - block * 60000).toLocaleString()}
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm text-gray-900">150 txns</div>
                    <div className="text-xs text-gray-500">1.2 MB</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </motion.div>

        {/* Recent Transactions */}
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.1 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200"
        >
          <div className="p-6 border-b border-gray-200">
            <h2 className="text-xl font-semibold text-gray-900">Recent Transactions</h2>
          </div>
          <div className="divide-y divide-gray-200">
            {[1, 2, 3, 4, 5].map((tx) => (
              <div key={tx} className="p-6 hover:bg-gray-50 transition-colors duration-200">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="font-mono text-sm text-gray-900">
                      abc123def456...ghi789
                    </div>
                    <div className="text-sm text-gray-600">
                      {new Date(Date.now() - tx * 30000).toLocaleString()}
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm text-gray-900">5.2500 UNI</div>
                    <div className="text-xs text-gray-500">0.001 fee</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  );
};

export default Explorer;
