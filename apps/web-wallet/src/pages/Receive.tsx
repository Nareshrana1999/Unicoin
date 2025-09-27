import React, { useState } from 'react';
import { motion } from 'framer-motion';
import QRCode from 'react-qr-code';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import toast from 'react-hot-toast';
import { useWallet } from '../hooks/useWallet';

const Receive: React.FC = () => {
  const { wallet, generateAddress } = useWallet();
  const [receiveAddress, setReceiveAddress] = useState('');
  const [amount, setAmount] = useState('');
  const [memo, setMemo] = useState('');

  React.useEffect(() => {
    if (wallet) {
      setReceiveAddress(wallet.address);
    }
  }, [wallet]);

  const handleGenerateNewAddress = async () => {
    try {
      const newAddress = await generateAddress();
      setReceiveAddress(newAddress);
      toast.success('New address generated!');
    } catch (error) {
      toast.error('Failed to generate new address');
    }
  };

  const qrValue = amount || memo 
    ? `unicoin:${receiveAddress}?amount=${amount}&memo=${encodeURIComponent(memo)}`
    : receiveAddress;

  if (!wallet) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className="text-center py-12">
          <div className="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <span className="text-4xl">🔒</span>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Wallet Not Connected</h2>
          <p className="text-gray-600 mb-8">
            Please connect your wallet to generate receive addresses.
          </p>
          <button className="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
            Connect Wallet
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Receive UNI</h1>
        <p className="text-gray-600">
          Share your address or QR code to receive Unicoin tokens.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* QR Code Section */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
        >
          <h2 className="text-xl font-semibold text-gray-900 mb-6">QR Code</h2>
          
          <div className="flex justify-center mb-6">
            <div className="bg-white p-4 rounded-lg shadow-inner">
              <QRCode
                value={qrValue}
                size={200}
                style={{ height: "auto", maxWidth: "100%", width: "100%" }}
              />
            </div>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Amount (Optional)
              </label>
              <input
                type="number"
                step="0.0001"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.0000"
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Memo (Optional)
              </label>
              <textarea
                value={memo}
                onChange={(e) => setMemo(e.target.value)}
                placeholder="Add a note for the sender..."
                rows={3}
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200 resize-none"
              />
            </div>
          </div>
        </motion.div>

        {/* Address Section */}
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.1 }}
          className="space-y-6"
        >
          {/* Current Address */}
          <div className="bg-white rounded-xl shadow-lg border border-gray-200 p-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Your Address</h2>
            
            <div className="bg-gray-50 rounded-lg p-4 mb-4">
              <p className="text-sm font-mono text-gray-700 break-all">
                {receiveAddress}
              </p>
            </div>

            <div className="flex space-x-3">
              <CopyToClipboard
                text={receiveAddress}
                onCopy={() => toast.success('Address copied to clipboard!')}
              >
                <button className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
                  Copy Address
                </button>
              </CopyToClipboard>
              
              <button
                onClick={handleGenerateNewAddress}
                className="flex-1 bg-gray-100 text-gray-700 py-2 px-4 rounded-lg font-medium hover:bg-gray-200 transition-colors duration-200"
              >
                New Address
              </button>
            </div>
          </div>

          {/* QR Code Data */}
          <div className="bg-white rounded-xl shadow-lg border border-gray-200 p-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">QR Code Data</h2>
            
            <div className="bg-gray-50 rounded-lg p-4 mb-4">
              <p className="text-sm font-mono text-gray-700 break-all">
                {qrValue}
              </p>
            </div>

            <CopyToClipboard
              text={qrValue}
              onCopy={() => toast.success('QR data copied to clipboard!')}
            >
              <button className="w-full bg-purple-600 text-white py-2 px-4 rounded-lg font-medium hover:bg-purple-700 transition-colors duration-200">
                Copy QR Data
              </button>
            </CopyToClipboard>
          </div>

          {/* Instructions */}
          <div className="bg-blue-50 rounded-xl border border-blue-200 p-6">
            <h3 className="text-lg font-semibold text-blue-900 mb-3">
              How to Receive UNI
            </h3>
            <ul className="space-y-2 text-blue-800">
              <li className="flex items-start space-x-2">
                <span className="text-blue-600 mt-1">1.</span>
                <span>Share your address or QR code with the sender</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-blue-600 mt-1">2.</span>
                <span>Wait for the transaction to be confirmed</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-blue-600 mt-1">3.</span>
                <span>Check your wallet balance and transaction history</span>
              </li>
            </ul>
          </div>

          {/* Security Tips */}
          <div className="bg-yellow-50 rounded-xl border border-yellow-200 p-6">
            <h3 className="text-lg font-semibold text-yellow-900 mb-3">
              Security Tips
            </h3>
            <ul className="space-y-2 text-yellow-800">
              <li className="flex items-start space-x-2">
                <span className="text-yellow-600 mt-1">•</span>
                <span>Always verify the address before sharing</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-yellow-600 mt-1">•</span>
                <span>Never share your private keys or seed phrase</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-yellow-600 mt-1">•</span>
                <span>Use a new address for each transaction when possible</span>
              </li>
            </ul>
          </div>
        </motion.div>
      </div>

      {/* Recent Addresses */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="mt-8 bg-white rounded-xl shadow-lg border border-gray-200 p-6"
      >
        <h2 className="text-xl font-semibold text-gray-900 mb-4">Recent Addresses</h2>
        <div className="space-y-3">
          {[
            { address: 'UNI1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh', balance: '1256.7890 UNI', used: '2 hours ago' },
            { address: 'UNI1abc123def456ghi789jkl012mno345pqr678stu', balance: '0.0000 UNI', used: '1 day ago' },
            { address: 'UNI1def456ghi789jkl012mno345pqr678stu901vwx', balance: '0.0000 UNI', used: '3 days ago' },
          ].map((addr, index) => (
            <div
              key={index}
              className="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition-colors duration-200"
            >
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-semibold text-sm">
                    {index + 1}
                  </span>
                </div>
                <div>
                  <div className="font-mono text-sm text-gray-700">
                    {addr.address.substring(0, 20)}...
                  </div>
                  <div className="text-xs text-gray-500">
                    Last used: {addr.used}
                  </div>
                </div>
              </div>
              <div className="text-right">
                <div className="font-medium text-gray-900">{addr.balance}</div>
                <div className="text-xs text-gray-500">Current balance</div>
              </div>
            </div>
          ))}
        </div>
      </motion.div>
    </div>
  );
};

export default Receive;
