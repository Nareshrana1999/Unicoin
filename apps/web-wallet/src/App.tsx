import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from 'react-query';
import { Toaster } from 'react-hot-toast';
import { motion } from 'framer-motion';

// Components
import Navbar from './components/Navbar';
import Sidebar from './components/Sidebar';
import Dashboard from './pages/Dashboard';
import Wallet from './pages/Wallet';
import Send from './pages/Send';
import Receive from './pages/Receive';
import History from './pages/History';
import Settings from './pages/Settings';
import Explorer from './pages/Explorer';
import DeFi from './pages/DeFi';
import NFT from './pages/NFT';

// Styles
import './App.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Router>
        <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
          <Toaster
            position="top-right"
            toastOptions={{
              duration: 4000,
              style: {
                background: '#363636',
                color: '#fff',
              },
              success: {
                duration: 3000,
                iconTheme: {
                  primary: '#10B981',
                  secondary: '#fff',
                },
              },
              error: {
                duration: 5000,
                iconTheme: {
                  primary: '#EF4444',
                  secondary: '#fff',
                },
              },
            }}
          />
          
          <Navbar />
          
          <div className="flex">
            <Sidebar />
            
            <main className="flex-1 ml-64 p-8">
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
                className="max-w-7xl mx-auto"
              >
                <Routes>
                  <Route path="/" element={<Dashboard />} />
                  <Route path="/wallet" element={<Wallet />} />
                  <Route path="/send" element={<Send />} />
                  <Route path="/receive" element={<Receive />} />
                  <Route path="/history" element={<History />} />
                  <Route path="/settings" element={<Settings />} />
                  <Route path="/explorer" element={<Explorer />} />
                  <Route path="/defi" element={<DeFi />} />
                  <Route path="/nft" element={<NFT />} />
                </Routes>
              </motion.div>
            </main>
          </div>
        </div>
      </Router>
    </QueryClientProvider>
  );
}

export default App;
