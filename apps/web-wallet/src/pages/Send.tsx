import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import toast from 'react-hot-toast';
import { useWallet } from '../hooks/useWallet';

const sendSchema = z.object({
  to: z.string().min(1, 'Recipient address is required'),
  amount: z.number().min(0.0001, 'Amount must be greater than 0'),
  fee: z.number().min(0.001, 'Fee must be at least 0.001 UNI'),
  memo: z.string().optional(),
});

type SendFormData = z.infer<typeof sendSchema>;

const Send: React.FC = () => {
  const { wallet, balance, sendTransaction, isLoading } = useWallet();
  const [isConfirming, setIsConfirming] = useState(false);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
    reset,
  } = useForm<SendFormData>({
    resolver: zodResolver(sendSchema),
    defaultValues: {
      amount: 0,
      fee: 0.001,
    },
  });

  const watchedAmount = watch('amount');
  const watchedFee = watch('fee');
  const totalCost = (watchedAmount || 0) + (watchedFee || 0);

  const onSubmit = async (data: SendFormData) => {
    if (!wallet) {
      toast.error('Please connect your wallet first');
      return;
    }

    if (totalCost > balance) {
      toast.error('Insufficient balance');
      return;
    }

    setIsConfirming(true);

    try {
      const transaction = await sendTransaction(data.to, data.amount, data.fee);
      toast.success(`Transaction sent! Hash: ${transaction.hash}`);
      reset();
    } catch (error) {
      toast.error(error instanceof Error ? error.message : 'Failed to send transaction');
    } finally {
      setIsConfirming(false);
    }
  };

  if (!wallet) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className="text-center py-12">
          <div className="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <span className="text-4xl">🔒</span>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Wallet Not Connected</h2>
          <p className="text-gray-600 mb-8">
            Please connect your wallet to send transactions.
          </p>
          <button className="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200">
            Connect Wallet
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Send UNI</h1>
        <p className="text-gray-600">
          Send Unicoin tokens to any address on the network.
        </p>
      </div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white rounded-xl shadow-lg border border-gray-200 p-8"
      >
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Recipient Address */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Recipient Address
            </label>
            <input
              type="text"
              {...register('to')}
              placeholder="Enter UNI address (e.g., UNI1abc123...)"
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
            />
            {errors.to && (
              <p className="text-red-600 text-sm mt-1">{errors.to.message}</p>
            )}
          </div>

          {/* Amount */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Amount (UNI)
            </label>
            <div className="relative">
              <input
                type="number"
                step="0.0001"
                {...register('amount', { valueAsNumber: true })}
                placeholder="0.0000"
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
              />
              <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
                <button
                  type="button"
                  onClick={() => {
                    const maxAmount = Math.max(0, balance - (watchedFee || 0.001));
                    // This would need to be implemented with setValue from react-hook-form
                  }}
                  className="text-blue-600 text-sm font-medium hover:text-blue-700"
                >
                  MAX
                </button>
              </div>
            </div>
            {errors.amount && (
              <p className="text-red-600 text-sm mt-1">{errors.amount.message}</p>
            )}
          </div>

          {/* Fee */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Transaction Fee (UNI)
            </label>
            <select
              {...register('fee', { valueAsNumber: true })}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200"
            >
              <option value={0.001}>Slow (0.001 UNI)</option>
              <option value={0.002}>Standard (0.002 UNI)</option>
              <option value={0.005}>Fast (0.005 UNI)</option>
            </select>
            {errors.fee && (
              <p className="text-red-600 text-sm mt-1">{errors.fee.message}</p>
            )}
          </div>

          {/* Memo */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Memo (Optional)
            </label>
            <textarea
              {...register('memo')}
              placeholder="Add a note to this transaction..."
              rows={3}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200 resize-none"
            />
          </div>

          {/* Transaction Summary */}
          <div className="bg-gray-50 rounded-lg p-4">
            <h3 className="font-medium text-gray-900 mb-3">Transaction Summary</h3>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-gray-600">Amount:</span>
                <span className="font-medium">{watchedAmount?.toFixed(4) || '0.0000'} UNI</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Fee:</span>
                <span className="font-medium">{watchedFee?.toFixed(4) || '0.0010'} UNI</span>
              </div>
              <div className="border-t border-gray-200 pt-2 flex justify-between font-semibold">
                <span className="text-gray-900">Total:</span>
                <span className="text-gray-900">{totalCost.toFixed(4)} UNI</span>
              </div>
            </div>
          </div>

          {/* Balance Info */}
          <div className="bg-blue-50 rounded-lg p-4">
            <div className="flex justify-between items-center">
              <span className="text-sm text-blue-800">Available Balance:</span>
              <span className="font-semibold text-blue-900">{balance.toFixed(4)} UNI</span>
            </div>
            {totalCost > balance && (
              <p className="text-red-600 text-sm mt-2">
                ⚠️ Insufficient balance for this transaction
              </p>
            )}
          </div>

          {/* Submit Button */}
          <button
            type="submit"
            disabled={isLoading || isConfirming || totalCost > balance}
            className="w-full bg-gradient-to-r from-blue-600 to-purple-600 text-white py-3 px-6 rounded-lg font-medium hover:from-blue-700 hover:to-purple-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105 disabled:hover:scale-100"
          >
            {isLoading || isConfirming ? (
              <div className="flex items-center justify-center space-x-2">
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                <span>{isConfirming ? 'Confirming...' : 'Sending...'}</span>
              </div>
            ) : (
              'Send Transaction'
            )}
          </button>
        </form>
      </motion.div>

      {/* Recent Recipients */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="mt-8 bg-white rounded-xl shadow-lg border border-gray-200 p-6"
      >
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Recent Recipients</h2>
        <div className="space-y-3">
          {[
            { name: 'Alice', address: 'UNI1alice123...', amount: '5.2500 UNI' },
            { name: 'Bob', address: 'UNI1bob456...', amount: '2.1000 UNI' },
            { name: 'Charlie', address: 'UNI1charlie789...', amount: '10.0000 UNI' },
          ].map((recipient, index) => (
            <button
              key={index}
              onClick={() => {
                // This would set the recipient address in the form
              }}
              className="w-full flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition-colors duration-200"
            >
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-semibold text-sm">
                    {recipient.name[0]}
                  </span>
                </div>
                <div className="text-left">
                  <div className="font-medium text-gray-900">{recipient.name}</div>
                  <div className="text-sm text-gray-600">{recipient.address}</div>
                </div>
              </div>
              <div className="text-sm text-gray-500">{recipient.amount}</div>
            </button>
          ))}
        </div>
      </motion.div>
    </div>
  );
};

export default Send;
