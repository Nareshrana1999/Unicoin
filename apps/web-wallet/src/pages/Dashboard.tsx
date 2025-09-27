import React from 'react';
import { motion } from 'framer-motion';
import {
  ArrowUpIcon,
  ArrowDownIcon,
  WalletIcon,
  ChartBarIcon,
  CurrencyDollarIcon,
  TrendingUpIcon,
} from '@heroicons/react/24/outline';
import { useWallet } from '../hooks/useWallet';

const Dashboard: React.FC = () => {
  const { wallet, balance, isConnected } = useWallet();

  const stats = [
    {
      name: 'Total Balance',
      value: `${balance.toFixed(4)} UNI`,
      change: '+12.5%',
      changeType: 'positive' as const,
      icon: WalletIcon,
      color: 'bg-gradient-to-r from-blue-500 to-blue-600',
    },
    {
      name: 'Portfolio Value',
      value: '$2,456.78',
      change: '+8.2%',
      changeType: 'positive' as const,
      icon: CurrencyDollarIcon,
      color: 'bg-gradient-to-r from-green-500 to-green-600',
    },
    {
      name: '24h Volume',
      value: '156.7 UNI',
      change: '-2.1%',
      changeType: 'negative' as const,
      icon: TrendingUpIcon,
      color: 'bg-gradient-to-r from-purple-500 to-purple-600',
    },
    {
      name: 'Active Addresses',
      value: '12',
      change: '+3',
      changeType: 'positive' as const,
      icon: ChartBarIcon,
      color: 'bg-gradient-to-r from-orange-500 to-orange-600',
    },
  ];

  const recentTransactions = [
    {
      id: '1',
      type: 'received',
      amount: '5.2500 UNI',
      from: 'UNI1abc123...',
      to: 'Your Wallet',
      timestamp: '2 minutes ago',
      status: 'confirmed',
    },
    {
      id: '2',
      type: 'sent',
      amount: '2.1000 UNI',
      from: 'Your Wallet',
      to: 'UNI1def456...',
      timestamp: '1 hour ago',
      status: 'confirmed',
    },
    {
      id: '3',
      type: 'received',
      amount: '10.0000 UNI',
      from: 'UNI1ghi789...',
      to: 'Your Wallet',
      timestamp: '3 hours ago',
      status: 'pending',
    },
  ];

  const quickActions = [
    {
      name: 'Send UNI',
      description: 'Send tokens to any address',
      icon: '📤',
      color: 'from-blue-500 to-blue-600',
      href: '/send',
    },
    {
      name: 'Receive UNI',
      description: 'Generate address to receive tokens',
      icon: '📥',
      color: 'from-green-500 to-green-600',
      href: '/receive',
    },
    {
      name: 'Stake UNI',
      description: 'Earn rewards by staking',
      icon: '🎯',
      color: 'from-purple-500 to-purple-600',
      href: '/defi',
    },
    {
      name: 'Explore',
      description: 'Browse blockchain data',
      icon: '🔍',
      color: 'from-orange-500 to-orange-600',
      href: '/explorer',
    },
  ];

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-gray-600 mt-2">
            Welcome back! Here's your Unicoin overview.
          </p>
        </div>
        
        {isConnected && (
          <div className="flex items-center space-x-3">
            <div className="text-right">
              <div className="text-sm text-gray-500">Connected as</div>
              <div className="font-semibold text-gray-900">{wallet?.name || 'Default Wallet'}</div>
            </div>
            <div className="w-12 h-12 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
              <span className="text-white font-bold text-lg">U</span>
            </div>
          </div>
        )}
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {stats.map((stat, index) => (
          <motion.div
            key={stat.name}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            className="bg-white rounded-xl shadow-lg p-6 border border-gray-100"
          >
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">{stat.name}</p>
                <p className="text-2xl font-bold text-gray-900 mt-2">{stat.value}</p>
                <div className="flex items-center mt-2">
                  {stat.changeType === 'positive' ? (
                    <ArrowUpIcon className="w-4 h-4 text-green-500 mr-1" />
                  ) : (
                    <ArrowDownIcon className="w-4 h-4 text-red-500 mr-1" />
                  )}
                  <span className={`text-sm font-medium ${
                    stat.changeType === 'positive' ? 'text-green-600' : 'text-red-600'
                  }`}>
                    {stat.change}
                  </span>
                  <span className="text-sm text-gray-500 ml-1">vs last month</span>
                </div>
              </div>
              <div className={`w-12 h-12 ${stat.color} rounded-lg flex items-center justify-center`}>
                <stat.icon className="w-6 h-6 text-white" />
              </div>
            </div>
          </motion.div>
        ))}
      </div>

      {/* Quick Actions */}
      <div>
        <h2 className="text-xl font-bold text-gray-900 mb-6">Quick Actions</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {quickActions.map((action, index) => (
            <motion.a
              key={action.name}
              href={action.href}
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: index * 0.1 }}
              className="group bg-white rounded-xl shadow-lg p-6 border border-gray-100 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1"
            >
              <div className="flex items-center space-x-4">
                <div className={`w-12 h-12 bg-gradient-to-r ${action.color} rounded-lg flex items-center justify-center group-hover:scale-110 transition-transform duration-200`}>
                  <span className="text-2xl">{action.icon}</span>
                </div>
                <div>
                  <h3 className="font-semibold text-gray-900 group-hover:text-blue-600 transition-colors duration-200">
                    {action.name}
                  </h3>
                  <p className="text-sm text-gray-600">{action.description}</p>
                </div>
              </div>
            </motion.a>
          ))}
        </div>
      </div>

      {/* Recent Transactions */}
      <div className="bg-white rounded-xl shadow-lg border border-gray-100">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-xl font-bold text-gray-900">Recent Transactions</h2>
          <p className="text-gray-600 mt-1">Your latest Unicoin activity</p>
        </div>
        
        <div className="divide-y divide-gray-200">
          {recentTransactions.map((tx, index) => (
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
                      {tx.type === 'received' ? 'Received' : 'Sent'} {tx.amount}
                    </div>
                    <div className="text-sm text-gray-600">
                      {tx.type === 'received' ? `From ${tx.from}` : `To ${tx.to}`}
                    </div>
                  </div>
                </div>
                
                <div className="text-right">
                  <div className="text-sm text-gray-500">{tx.timestamp}</div>
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
        
        <div className="p-6 border-t border-gray-200">
          <a
            href="/history"
            className="inline-flex items-center text-blue-600 hover:text-blue-700 font-medium transition-colors duration-200"
          >
            View all transactions
            <ArrowUpIcon className="w-4 h-4 ml-1 rotate-90" />
          </a>
        </div>
      </div>

      {/* Market Overview */}
      <div className="bg-white rounded-xl shadow-lg border border-gray-100 p-6">
        <h2 className="text-xl font-bold text-gray-900 mb-6">Market Overview</h2>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="text-center">
            <div className="text-2xl font-bold text-gray-900">$0.2456</div>
            <div className="text-sm text-gray-600">UNI Price</div>
            <div className="text-sm text-green-600 mt-1">+5.2% 24h</div>
          </div>
          
          <div className="text-center">
            <div className="text-2xl font-bold text-gray-900">$51.6M</div>
            <div className="text-sm text-gray-600">Market Cap</div>
            <div className="text-sm text-green-600 mt-1">+8.1% 24h</div>
          </div>
          
          <div className="text-center">
            <div className="text-2xl font-bold text-gray-900">210K</div>
            <div className="text-sm text-gray-600">24h Volume</div>
            <div className="text-sm text-red-600 mt-1">-2.3% 24h</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
