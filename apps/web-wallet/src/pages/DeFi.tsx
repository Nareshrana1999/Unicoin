import React from 'react';
import { motion } from 'framer-motion';

const DeFi: React.FC = () => {
  return (
    <div className="max-w-7xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">DeFi Protocols</h1>
        <p className="text-gray-600">
          Access decentralized finance protocols and earn rewards with your UNI tokens.
        </p>
      </div>

      {/* DeFi Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        {[
          { name: 'Total Value Locked', value: '$2.45M', change: '+12.5%' },
          { name: 'Active Pools', value: '24', change: '+3' },
          { name: 'APY Average', value: '15.6%', change: '+2.1%' },
          { name: 'Total Users', value: '1,234', change: '+89' },
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

      {/* DeFi Protocols */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {[
          {
            name: 'UniSwap DEX',
            description: 'Decentralized exchange for trading UNI tokens',
            apy: '12.5%',
            tvl: '$456K',
            icon: '🔄',
            color: 'from-blue-500 to-blue-600',
          },
          {
            name: 'UniLend',
            description: 'Lending and borrowing protocol',
            apy: '18.2%',
            tvl: '$789K',
            icon: '💰',
            color: 'from-green-500 to-green-600',
          },
          {
            name: 'UniFarm',
            description: 'Yield farming and staking rewards',
            apy: '25.8%',
            tvl: '$1.2M',
            icon: '🌾',
            color: 'from-purple-500 to-purple-600',
          },
          {
            name: 'UniStake',
            description: 'Stake UNI tokens and earn rewards',
            apy: '8.5%',
            tvl: '$2.1M',
            icon: '🎯',
            color: 'from-orange-500 to-orange-600',
          },
          {
            name: 'UniBridge',
            description: 'Cross-chain bridge for multi-asset support',
            apy: '6.2%',
            tvl: '$234K',
            icon: '🌉',
            color: 'from-teal-500 to-teal-600',
          },
          {
            name: 'UniInsurance',
            description: 'DeFi insurance and risk management',
            apy: '4.8%',
            tvl: '$156K',
            icon: '🛡️',
            color: 'from-red-500 to-red-600',
          },
        ].map((protocol, index) => (
          <motion.div
            key={protocol.name}
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: index * 0.1 }}
            className="bg-white rounded-xl shadow-lg border border-gray-200 p-6 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1"
          >
            <div className="flex items-center space-x-4 mb-4">
              <div className={`w-12 h-12 bg-gradient-to-r ${protocol.color} rounded-lg flex items-center justify-center`}>
                <span className="text-2xl">{protocol.icon}</span>
              </div>
              <div>
                <h3 className="font-semibold text-gray-900">{protocol.name}</h3>
                <p className="text-sm text-gray-600">{protocol.description}</p>
              </div>
            </div>
            
            <div className="grid grid-cols-2 gap-4 mb-6">
              <div>
                <div className="text-xs text-gray-500">APY</div>
                <div className="font-semibold text-green-600">{protocol.apy}</div>
              </div>
              <div>
                <div className="text-xs text-gray-500">TVL</div>
                <div className="font-semibold text-gray-900">{protocol.tvl}</div>
              </div>
            </div>
            
            <button className={`w-full bg-gradient-to-r ${protocol.color} text-white py-2 px-4 rounded-lg font-medium hover:opacity-90 transition-opacity duration-200`}>
              Enter Protocol
            </button>
          </motion.div>
        ))}
      </div>
    </div>
  );
};

export default DeFi;
