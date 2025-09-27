import React, { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
  Bars3Icon,
  BellIcon,
  Cog6ToothIcon,
  UserCircleIcon,
  WalletIcon,
} from '@heroicons/react/24/outline';
import { useWallet } from '../hooks/useWallet';

const Navbar: React.FC = () => {
  const [isProfileOpen, setIsProfileOpen] = useState(false);
  const location = useLocation();
  const { wallet, balance, isConnected } = useWallet();

  const navigation = [
    { name: 'Dashboard', href: '/', icon: '🏠' },
    { name: 'Wallet', href: '/wallet', icon: '💼' },
    { name: 'Send', href: '/send', icon: '📤' },
    { name: 'Receive', href: '/receive', icon: '📥' },
    { name: 'History', href: '/history', icon: '📋' },
    { name: 'Explorer', href: '/explorer', icon: '🔍' },
    { name: 'DeFi', href: '/defi', icon: '🏦' },
    { name: 'NFT', href: '/nft', icon: '🎨' },
    { name: 'Settings', href: '/settings', icon: '⚙️' },
  ];

  return (
    <nav className="bg-white shadow-lg border-b border-gray-200 fixed w-full top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          {/* Logo and Brand */}
          <div className="flex items-center">
            <Link to="/" className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg flex items-center justify-center">
                <span className="text-white font-bold text-sm">U</span>
              </div>
              <span className="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                Unicoin
              </span>
            </Link>
          </div>

          {/* Navigation Links */}
          <div className="hidden md:flex items-center space-x-8">
            {navigation.map((item) => (
              <Link
                key={item.name}
                to={item.href}
                className={`flex items-center space-x-2 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 ${
                  location.pathname === item.href
                    ? 'text-blue-600 bg-blue-50'
                    : 'text-gray-700 hover:text-blue-600 hover:bg-gray-50'
                }`}
              >
                <span className="text-lg">{item.icon}</span>
                <span>{item.name}</span>
              </Link>
            ))}
          </div>

          {/* Right side */}
          <div className="flex items-center space-x-4">
            {/* Balance Display */}
            {isConnected && (
              <motion.div
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                className="hidden lg:flex items-center space-x-2 bg-gradient-to-r from-green-50 to-emerald-50 px-4 py-2 rounded-lg border border-green-200"
              >
                <WalletIcon className="w-5 h-5 text-green-600" />
                <span className="text-sm font-medium text-green-800">
                  {balance.toFixed(4)} UNI
                </span>
              </motion.div>
            )}

            {/* Notifications */}
            <button className="p-2 text-gray-400 hover:text-gray-600 transition-colors duration-200">
              <BellIcon className="w-6 h-6" />
            </button>

            {/* Profile Menu */}
            <div className="relative">
              <button
                onClick={() => setIsProfileOpen(!isProfileOpen)}
                className="flex items-center space-x-2 p-2 text-gray-700 hover:text-blue-600 transition-colors duration-200"
              >
                <UserCircleIcon className="w-8 h-8" />
                <span className="hidden md:block text-sm font-medium">
                  {wallet?.name || 'Guest'}
                </span>
              </button>

              {isProfileOpen && (
                <motion.div
                  initial={{ opacity: 0, y: -10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg border border-gray-200 py-1 z-50"
                >
                  <Link
                    to="/settings"
                    className="flex items-center space-x-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
                    onClick={() => setIsProfileOpen(false)}
                  >
                    <Cog6ToothIcon className="w-4 h-4" />
                    <span>Settings</span>
                  </Link>
                  <button className="w-full text-left flex items-center space-x-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50">
                    <span>🔓</span>
                    <span>Lock Wallet</span>
                  </button>
                  <hr className="my-1" />
                  <button className="w-full text-left flex items-center space-x-2 px-4 py-2 text-sm text-red-600 hover:bg-red-50">
                    <span>🚪</span>
                    <span>Disconnect</span>
                  </button>
                </motion.div>
              )}
            </div>

            {/* Mobile menu button */}
            <button className="md:hidden p-2 text-gray-400 hover:text-gray-600">
              <Bars3Icon className="w-6 h-6" />
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
