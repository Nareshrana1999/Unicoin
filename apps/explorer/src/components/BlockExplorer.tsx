import React, { useState, useEffect } from 'react';
import styled from 'styled-components';
import { motion } from 'framer-motion';
import { useQuery } from 'react-query';
import { toast } from 'react-hot-toast';

const ExplorerContainer = styled(motion.div)`
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem;
  background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
  min-height: 100vh;
  color: white;
`;

const ExplorerHeader = styled.div`
  text-align: center;
  margin-bottom: 3rem;
`;

const ExplorerTitle = styled.h1`
  font-size: 3.5rem;
  font-weight: 700;
  margin-bottom: 1rem;
  text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
  background: linear-gradient(45deg, #3498db, #2ecc71);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
`;

const ExplorerSubtitle = styled.p`
  font-size: 1.3rem;
  opacity: 0.9;
  margin-bottom: 2rem;
`;

const SearchBar = styled.div`
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  justify-content: center;
  flex-wrap: wrap;
`;

const SearchInput = styled.input`
  padding: 1rem 1.5rem;
  border: 2px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 1rem;
  min-width: 300px;
  backdrop-filter: blur(10px);
  
  &::placeholder {
    color: rgba(255, 255, 255, 0.7);
  }
  
  &:focus {
    outline: none;
    border-color: #3498db;
  }
`;

const SearchButton = styled(motion.button)`
  background: linear-gradient(135deg, #3498db 0%, #2ecc71 100%);
  color: white;
  border: none;
  border-radius: 12px;
  padding: 1rem 2rem;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  
  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 20px rgba(0,0,0,0.2);
  }
`;

const StatsGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin-bottom: 3rem;
`;

const StatCard = styled(motion.div)`
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  padding: 2rem;
  text-align: center;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
`;

const StatValue = styled.div`
  font-size: 2.5rem;
  font-weight: 700;
  color: #3498db;
  margin-bottom: 0.5rem;
`;

const StatLabel = styled.div`
  font-size: 1.1rem;
  opacity: 0.9;
`;

const Section = styled(motion.div)`
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  padding: 2rem;
  margin-bottom: 2rem;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
`;

const SectionTitle = styled.h2`
  font-size: 2rem;
  margin-bottom: 1.5rem;
  color: #3498db;
  text-align: center;
`;

const Table = styled.table`
  width: 100%;
  border-collapse: collapse;
  margin-top: 1rem;
`;

const TableHeader = styled.th`
  background: rgba(52, 152, 219, 0.3);
  padding: 1rem;
  text-align: left;
  font-weight: 600;
  border-bottom: 2px solid rgba(255, 255, 255, 0.2);
`;

const TableCell = styled.td`
  padding: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
`;

const TableRow = styled(motion.tr)`
  &:hover {
    background: rgba(255, 255, 255, 0.05);
  }
`;

const BlockHash = styled.code`
  background: rgba(0, 0, 0, 0.3);
  padding: 0.3rem 0.6rem;
  border-radius: 6px;
  font-size: 0.9rem;
  color: #2ecc71;
`;

const Address = styled.code`
  background: rgba(0, 0, 0, 0.3);
  padding: 0.3rem 0.6rem;
  border-radius: 6px;
  font-size: 0.9rem;
  color: #e74c3c;
`;

const Amount = styled.span`
  font-weight: 600;
  color: #f39c12;
`;

const StatusBadge = styled.span<{ status: string }>`
  padding: 0.3rem 0.8rem;
  border-radius: 20px;
  font-size: 0.8rem;
  font-weight: 600;
  background: ${props => 
    props.status === 'confirmed' ? '#27ae60' :
    props.status === 'pending' ? '#f39c12' : '#e74c3c'
  };
`;

const LoadingSpinner = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  font-size: 1.2rem;
`;

const Pagination = styled.div`
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 2rem;
`;

const PageButton = styled(motion.button)<{ active?: boolean }>`
  background: ${props => props.active ? '#3498db' : 'rgba(255, 255, 255, 0.1)'};
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.8rem 1.2rem;
  cursor: pointer;
  transition: all 0.3s ease;
  
  &:hover {
    background: #3498db;
    transform: translateY(-2px);
  }
`;

interface BlockData {
  hash: string;
  height: number;
  timestamp: number;
  transactions: number;
  size: number;
  miner: string;
}

interface TransactionData {
  hash: string;
  blockHash: string;
  from: string;
  to: string;
  amount: number;
  fee: number;
  timestamp: number;
  status: 'confirmed' | 'pending' | 'failed';
}

interface ExplorerStats {
  totalBlocks: number;
  totalTransactions: number;
  totalVolume: number;
  networkHashRate: string;
  difficulty: number;
  averageBlockTime: number;
}

const BlockExplorer: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [searchType, setSearchType] = useState<'block' | 'transaction' | 'address'>('block');

  // Mock data fetching - in real implementation, these would be API calls
  const { data: stats, isLoading: statsLoading } = useQuery<ExplorerStats>('explorer-stats', async () => {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 1000));
    return {
      totalBlocks: 1234567,
      totalTransactions: 9876543,
      totalVolume: 21000000.5,
      networkHashRate: '125.7 TH/s',
      difficulty: 1856234567890,
      averageBlockTime: 10.2,
    };
  });

  const { data: blocks, isLoading: blocksLoading } = useQuery<BlockData[]>('recent-blocks', async () => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return Array.from({ length: 10 }, (_, i) => ({
      hash: `0x${Math.random().toString(16).substr(2, 64)}`,
      height: stats?.totalBlocks ? stats.totalBlocks - i : 1234567 - i,
      timestamp: Date.now() - (i * 10000),
      transactions: Math.floor(Math.random() * 1000) + 100,
      size: Math.floor(Math.random() * 1000000) + 500000,
      miner: `0x${Math.random().toString(16).substr(2, 40)}`,
    }));
  });

  const { data: transactions, isLoading: transactionsLoading } = useQuery<TransactionData[]>('recent-transactions', async () => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return Array.from({ length: 15 }, (_, i) => ({
      hash: `0x${Math.random().toString(16).substr(2, 64)}`,
      blockHash: `0x${Math.random().toString(16).substr(2, 64)}`,
      from: `0x${Math.random().toString(16).substr(2, 40)}`,
      to: `0x${Math.random().toString(16).substr(2, 40)}`,
      amount: Math.random() * 1000 + 0.001,
      fee: Math.random() * 0.01 + 0.0001,
      timestamp: Date.now() - (i * 5000),
      status: ['confirmed', 'pending', 'failed'][Math.floor(Math.random() * 3)] as any,
    }));
  });

  const handleSearch = () => {
    if (!searchTerm.trim()) {
      toast.error('Please enter a search term');
      return;
    }

    // In real implementation, this would trigger a search API call
    toast.success(`Searching for ${searchType}: ${searchTerm}`);
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString();
  };

  const formatHash = (hash: string, length: number = 12) => {
    return `${hash.slice(0, length)}...${hash.slice(-length)}`;
  };

  const formatAmount = (amount: number) => {
    return amount.toLocaleString('en-US', {
      minimumFractionDigits: 9,
      maximumFractionDigits: 9,
    });
  };

  const formatFileSize = (bytes: number) => {
    const sizes = ['B', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  };

  return (
    <ExplorerContainer
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <ExplorerHeader>
        <ExplorerTitle>Unicoin Explorer</ExplorerTitle>
        <ExplorerSubtitle>
          Explore the Unicoin blockchain in real-time
        </ExplorerSubtitle>
      </ExplorerHeader>

      <SearchBar>
        <select 
          value={searchType} 
          onChange={(e) => setSearchType(e.target.value as any)}
          style={{
            padding: '1rem',
            border: '2px solid rgba(255, 255, 255, 0.2)',
            borderRadius: '12px',
            background: 'rgba(255, 255, 255, 0.1)',
            color: 'white',
            fontSize: '1rem',
            backdropFilter: 'blur(10px)',
          }}
        >
          <option value="block">Block</option>
          <option value="transaction">Transaction</option>
          <option value="address">Address</option>
        </select>
        <SearchInput
          type="text"
          placeholder={`Search for ${searchType} hash or address...`}
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
        />
        <SearchButton
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={handleSearch}
        >
          Search
        </SearchButton>
      </SearchBar>

      {statsLoading ? (
        <LoadingSpinner>Loading blockchain statistics...</LoadingSpinner>
      ) : (
        <StatsGrid>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.1 }}
          >
            <StatValue>{stats?.totalBlocks.toLocaleString()}</StatValue>
            <StatLabel>Total Blocks</StatLabel>
          </StatCard>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.2 }}
          >
            <StatValue>{stats?.totalTransactions.toLocaleString()}</StatValue>
            <StatLabel>Total Transactions</StatLabel>
          </StatCard>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.3 }}
          >
            <StatValue>{formatAmount(stats?.totalVolume || 0)}</StatValue>
            <StatLabel>Total Volume (UNI)</StatLabel>
          </StatCard>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.4 }}
          >
            <StatValue>{stats?.networkHashRate}</StatValue>
            <StatLabel>Network Hash Rate</StatLabel>
          </StatCard>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.5 }}
          >
            <StatValue>{stats?.difficulty.toExponential(2)}</StatValue>
            <StatLabel>Difficulty</StatLabel>
          </StatCard>
          <StatCard
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.6 }}
          >
            <StatValue>{stats?.averageBlockTime}s</StatValue>
            <StatLabel>Avg Block Time</StatLabel>
          </StatCard>
        </StatsGrid>
      )}

      <Section
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.7 }}
      >
        <SectionTitle>Recent Blocks</SectionTitle>
        {blocksLoading ? (
          <LoadingSpinner>Loading recent blocks...</LoadingSpinner>
        ) : (
          <Table>
            <thead>
              <tr>
                <TableHeader>Height</TableHeader>
                <TableHeader>Hash</TableHeader>
                <TableHeader>Time</TableHeader>
                <TableHeader>Transactions</TableHeader>
                <TableHeader>Size</TableHeader>
                <TableHeader>Miner</TableHeader>
              </tr>
            </thead>
            <tbody>
              {blocks?.map((block, index) => (
                <TableRow
                  key={block.hash}
                  initial={{ x: -50, opacity: 0 }}
                  animate={{ x: 0, opacity: 1 }}
                  transition={{ delay: index * 0.1 }}
                >
                  <TableCell>{block.height.toLocaleString()}</TableCell>
                  <TableCell>
                    <BlockHash>{formatHash(block.hash)}</BlockHash>
                  </TableCell>
                  <TableCell>{formatTimestamp(block.timestamp)}</TableCell>
                  <TableCell>{block.transactions.toLocaleString()}</TableCell>
                  <TableCell>{formatFileSize(block.size)}</TableCell>
                  <TableCell>
                    <Address>{formatHash(block.miner, 8)}</Address>
                  </TableCell>
                </TableRow>
              ))}
            </tbody>
          </Table>
        )}
      </Section>

      <Section
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.9 }}
      >
        <SectionTitle>Recent Transactions</SectionTitle>
        {transactionsLoading ? (
          <LoadingSpinner>Loading recent transactions...</LoadingSpinner>
        ) : (
          <Table>
            <thead>
              <tr>
                <TableHeader>Hash</TableHeader>
                <TableHeader>From</TableHeader>
                <TableHeader>To</TableHeader>
                <TableHeader>Amount</TableHeader>
                <TableHeader>Fee</TableHeader>
                <TableHeader>Time</TableHeader>
                <TableHeader>Status</TableHeader>
              </tr>
            </thead>
            <tbody>
              {transactions?.map((tx, index) => (
                <TableRow
                  key={tx.hash}
                  initial={{ x: -50, opacity: 0 }}
                  animate={{ x: 0, opacity: 1 }}
                  transition={{ delay: index * 0.05 }}
                >
                  <TableCell>
                    <BlockHash>{formatHash(tx.hash)}</BlockHash>
                  </TableCell>
                  <TableCell>
                    <Address>{formatHash(tx.from, 8)}</Address>
                  </TableCell>
                  <TableCell>
                    <Address>{formatHash(tx.to, 8)}</Address>
                  </TableCell>
                  <TableCell>
                    <Amount>{formatAmount(tx.amount)} UNI</Amount>
                  </TableCell>
                  <TableCell>{formatAmount(tx.fee)} UNI</TableCell>
                  <TableCell>{formatTimestamp(tx.timestamp)}</TableCell>
                  <TableCell>
                    <StatusBadge status={tx.status}>{tx.status}</StatusBadge>
                  </TableCell>
                </TableRow>
              ))}
            </tbody>
          </Table>
        )}
      </Section>

      <Pagination>
        <PageButton
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={() => setCurrentPage(Math.max(1, currentPage - 1))}
          disabled={currentPage === 1}
        >
          Previous
        </PageButton>
        <PageButton
          active={true}
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
        >
          {currentPage}
        </PageButton>
        <PageButton
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={() => setCurrentPage(currentPage + 1)}
        >
          Next
        </PageButton>
      </Pagination>
    </ExplorerContainer>
  );
};

export default BlockExplorer;
