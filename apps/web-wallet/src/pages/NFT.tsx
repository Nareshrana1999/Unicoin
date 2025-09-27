import React from 'react';
import { motion } from 'framer-motion';

const NFT: React.FC = () => {
  return (
    <div className="max-w-7xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">NFT Marketplace</h1>
        <p className="text-gray-600">
          Create, buy, and sell NFTs on the Unicoin blockchain.
        </p>
      </div>

      {/* NFT Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        {[
          { name: 'Total NFTs', value: '45,678', change: '+1,234' },
          { name: 'Collections', value: '156', change: '+12' },
          { name: 'Volume (24h)', value: '2.3K UNI', change: '+15.2%' },
          { name: 'Floor Price', value: '0.5 UNI', change: '+8.7%' },
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

      {/* Featured Collections */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold text-gray-900 mb-6">Featured Collections</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {[
            {
              name: 'UniCats',
              description: '10,000 unique cat NFTs',
              floorPrice: '0.8 UNI',
              volume: '456 UNI',
              image: '🐱',
              color: 'from-pink-500 to-pink-600',
            },
            {
              name: 'UniArt',
              description: 'Digital art collection',
              floorPrice: '1.2 UNI',
              volume: '789 UNI',
              image: '🎨',
              color: 'from-purple-500 to-purple-600',
            },
            {
              name: 'UniGames',
              description: 'Gaming assets and items',
              floorPrice: '0.6 UNI',
              volume: '234 UNI',
              image: '🎮',
              color: 'from-blue-500 to-blue-600',
            },
          ].map((collection, index) => (
            <motion.div
              key={collection.name}
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: index * 0.1 }}
              className="bg-white rounded-xl shadow-lg border border-gray-200 p-6 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1"
            >
              <div className="text-center">
                <div className={`w-20 h-20 bg-gradient-to-r ${collection.color} rounded-xl flex items-center justify-center mx-auto mb-4`}>
                  <span className="text-4xl">{collection.image}</span>
                </div>
                <h3 className="font-semibold text-gray-900 mb-2">{collection.name}</h3>
                <p className="text-sm text-gray-600 mb-4">{collection.description}</p>
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div>
                    <div className="text-xs text-gray-500">Floor Price</div>
                    <div className="font-semibold text-gray-900">{collection.floorPrice}</div>
                  </div>
                  <div>
                    <div className="text-xs text-gray-500">Volume</div>
                    <div className="font-semibold text-gray-900">{collection.volume}</div>
                  </div>
                </div>
                <button className={`w-full bg-gradient-to-r ${collection.color} text-white py-2 px-4 rounded-lg font-medium hover:opacity-90 transition-opacity duration-200`}>
                  View Collection
                </button>
              </div>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Recent NFTs */}
      <div>
        <h2 className="text-xl font-semibold text-gray-900 mb-6">Recent NFTs</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {[1, 2, 3, 4, 5, 6, 7, 8].map((nft) => (
            <motion.div
              key={nft}
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: nft * 0.1 }}
              className="bg-white rounded-xl shadow-lg border border-gray-200 overflow-hidden hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1"
            >
              <div className="aspect-square bg-gradient-to-br from-blue-400 to-purple-500 flex items-center justify-center">
                <span className="text-6xl">🎨</span>
              </div>
              <div className="p-4">
                <h3 className="font-semibold text-gray-900 mb-1">UniCat #{nft}</h3>
                <p className="text-sm text-gray-600 mb-3">UniCats Collection</p>
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-xs text-gray-500">Price</div>
                    <div className="font-semibold text-gray-900">0.8 UNI</div>
                  </div>
                  <button className="bg-blue-600 text-white px-3 py-1 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors duration-200">
                    Buy
                  </button>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default NFT;
