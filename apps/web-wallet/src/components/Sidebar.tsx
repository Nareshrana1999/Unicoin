import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { motion } from 'framer-motion';

const Sidebar: React.FC = () => {
  const location = useLocation();

  const navigation = [
    { name: 'Dashboard', href: '/', icon: '🏠', description: 'Overview & Analytics' },
    { name: 'Wallet', href: '/wallet', icon: '💼', description: 'Manage Addresses' },
    { name: 'Send', href: '/send', icon: '📤', description: 'Send UNI Tokens' },
    { name: 'Receive', href: '/receive', icon: '📥', description: 'Receive UNI Tokens' },
    { name: 'History', href: '/history', icon: '📋', description: 'Transaction History' },
    { name: 'Explorer', href: '/explorer', icon: '🔍', description: 'Block Explorer' },
    { name: 'DeFi', href: '/defi', icon: '🏦', description: 'DeFi Protocols' },
    { name: 'NFT', href: '/nft', icon: '🎨', description: 'NFT Marketplace' },
    { name: 'Settings', href: '/settings', icon: '⚙️', description: 'Wallet Settings' },
  ];

  return (
    <aside className="fixed left-0 top-16 h-full w-64 bg-white shadow-lg border-r border-gray-200 overflow-y-auto z-40">
      <div className="p-6">
        <h2 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-4">
          Navigation
        </h2>
        
        <nav className="space-y-2">
          {navigation.map((item, index) => {
            const isActive = location.pathname === item.href;
            
            return (
              <motion.div
                key={item.name}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: index * 0.1 }}
              >
                <Link
                  to={item.href}
                  className={`group flex items-center px-4 py-3 text-sm font-medium rounded-lg transition-all duration-200 ${
                    isActive
                      ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg'
                      : 'text-gray-700 hover:bg-gray-100 hover:text-blue-600'
                  }`}
                >
                  <span className={`text-xl mr-3 ${isActive ? 'text-white' : 'text-gray-400 group-hover:text-blue-500'}`}>
                    {item.icon}
                  </span>
                  <div className="flex-1">
                    <div className="font-medium">{item.name}</div>
                    <div className={`text-xs ${isActive ? 'text-blue-100' : 'text-gray-500'}`}>
                      {item.description}
                    </div>
                  </div>
                  {isActive && (
                    <motion.div
                      initial={{ scale: 0 }}
                      animate={{ scale: 1 }}
                      className="w-2 h-2 bg-white rounded-full"
                    />
                  )}
                </Link>
              </motion.div>
            );
          })}
        </nav>

        {/* Quick Actions */}
        <div className="mt-8">
          <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-4">
            Quick Actions
          </h3>
          
          <div className="space-y-2">
            <button className="w-full flex items-center px-4 py-3 text-sm font-medium text-gray-700 bg-green-50 hover:bg-green-100 rounded-lg transition-colors duration-200">
              <span className="text-xl mr-3">⚡</span>
              <span>Quick Send</span>
            </button>
            
            <button className="w-full flex items-center px-4 py-3 text-sm font-medium text-gray-700 bg-blue-50 hover:bg-blue-100 rounded-lg transition-colors duration-200">
              <span className="text-xl mr-3">📊</span>
              <span>Portfolio</span>
            </button>
            
            <button className="w-full flex items-center px-4 py-3 text-sm font-medium text-gray-700 bg-purple-50 hover:bg-purple-100 rounded-lg transition-colors duration-200">
              <span className="text-xl mr-3">🎯</span>
              <span>Staking</span>
            </button>
          </div>
        </div>

        {/* Network Status */}
        <div className="mt-8 p-4 bg-gray-50 rounded-lg">
          <div className="flex items-center justify-between mb-2">
            <span className="text-xs font-medium text-gray-600">Network</span>
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
              <span className="text-xs text-green-600 font-medium">Connected</span>
            </div>
          </div>
          <div className="text-xs text-gray-500">
            Unicoin Mainnet
          </div>
          <div className="text-xs text-gray-500">
            Block: 1,234,567
          </div>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
