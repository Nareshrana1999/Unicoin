import React, { useState, useEffect } from 'react';
import styled from 'styled-components';
import { motion } from 'framer-motion';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../store/store';
import { 
  sendTransaction, 
  receiveTransaction, 
  updateBalance,
  generateAddress 
} from '../store/walletSlice';
import { toast } from 'react-hot-toast';

const WalletContainer = styled(motion.div)`
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
`;

const WalletHeader = styled.div`
  text-align: center;
  margin-bottom: 3rem;
  color: white;
`;

const WalletTitle = styled.h1`
  font-size: 3rem;
  font-weight: 700;
  margin-bottom: 1rem;
  text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
`;

const WalletSubtitle = styled.p`
  font-size: 1.2rem;
  opacity: 0.9;
  margin-bottom: 2rem;
`;

const BalanceCard = styled(motion.div)`
  background: rgba(255, 255, 255, 0.95);
  border-radius: 20px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 20px 40px rgba(0,0,0,0.1);
  backdrop-filter: blur(10px);
`;

const BalanceTitle = styled.h2`
  font-size: 1.5rem;
  color: #333;
  margin-bottom: 1rem;
  text-align: center;
`;

const BalanceAmount = styled.div`
  font-size: 3rem;
  font-weight: 700;
  color: #667eea;
  text-align: center;
  margin-bottom: 1rem;
`;

const BalanceUSD = styled.div`
  font-size: 1.2rem;
  color: #666;
  text-align: center;
  margin-bottom: 2rem;
`;

const ActionButtons = styled.div`
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
`;

const ActionButton = styled(motion.button)`
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 12px;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  
  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 20px rgba(0,0,0,0.2);
  }
`;

const TransactionSection = styled.div`
  background: rgba(255, 255, 255, 0.95);
  border-radius: 20px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 20px 40px rgba(0,0,0,0.1);
`;

const SectionTitle = styled.h3`
  font-size: 1.8rem;
  color: #333;
  margin-bottom: 1.5rem;
  text-align: center;
`;

const FormGroup = styled.div`
  margin-bottom: 1.5rem;
`;

const Label = styled.label`
  display: block;
  font-size: 1.1rem;
  color: #333;
  margin-bottom: 0.5rem;
  font-weight: 600;
`;

const Input = styled.input`
  width: 100%;
  padding: 1rem;
  border: 2px solid #e1e5e9;
  border-radius: 12px;
  font-size: 1rem;
  transition: border-color 0.3s ease;
  
  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

const TransactionList = styled.div`
  max-height: 400px;
  overflow-y: auto;
`;

const TransactionItem = styled(motion.div)`
  background: #f8f9fa;
  border-radius: 12px;
  padding: 1rem;
  margin-bottom: 1rem;
  border-left: 4px solid ${props => props.color || '#667eea'};
`;

const TransactionDetails = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
`;

const TransactionAmount = styled.span`
  font-size: 1.2rem;
  font-weight: 700;
  color: ${props => props.color || '#333'};
`;

const TransactionInfo = styled.div`
  font-size: 0.9rem;
  color: #666;
`;

const AddressDisplay = styled.div`
  background: #f8f9fa;
  border-radius: 12px;
  padding: 1rem;
  margin: 1rem 0;
  word-break: break-all;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
`;

const CopyButton = styled(motion.button)`
  background: #667eea;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.5rem 1rem;
  font-size: 0.9rem;
  cursor: pointer;
  margin-left: 1rem;
`;

interface WalletProps {}

const Wallet: React.FC<WalletProps> = () => {
  const dispatch = useDispatch();
  const { balance, address, transactions, isLoading } = useSelector((state: RootState) => state.wallet);
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [showReceive, setShowReceive] = useState(false);
  const [newAddress, setNewAddress] = useState('');

  useEffect(() => {
    // Initialize wallet on component mount
    dispatch(updateBalance());
  }, [dispatch]);

  const handleSendTransaction = async () => {
    if (!recipient || !amount) {
      toast.error('Please fill in all fields');
      return;
    }

    try {
      await dispatch(sendTransaction({
        recipient,
        amount: parseFloat(amount),
      }));
      toast.success('Transaction sent successfully!');
      setRecipient('');
      setAmount('');
    } catch (error) {
      toast.error('Failed to send transaction');
    }
  };

  const handleGenerateAddress = () => {
    const address = dispatch(generateAddress());
    setNewAddress(address as any);
    setShowReceive(true);
    toast.success('New address generated!');
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    toast.success('Copied to clipboard!');
  };

  const formatBalance = (balance: number) => {
    return balance.toLocaleString('en-US', {
      minimumFractionDigits: 9,
      maximumFractionDigits: 9,
    });
  };

  const formatUSD = (balance: number) => {
    // Assuming 1 UNI = $1 for demo purposes
    const usdValue = balance * 1;
    return `$${usdValue.toLocaleString('en-US', { minimumFractionDigits: 2 })}`;
  };

  return (
    <WalletContainer
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <WalletHeader>
        <WalletTitle>Unicoin Wallet</WalletTitle>
        <WalletSubtitle>
          The Ultimate Secure Cryptocurrency Wallet with Privacy Features
        </WalletSubtitle>
      </WalletHeader>

      <BalanceCard
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.2 }}
      >
        <BalanceTitle>Your Balance</BalanceTitle>
        <BalanceAmount>{formatBalance(balance)} UNI</BalanceAmount>
        <BalanceUSD>{formatUSD(balance)}</BalanceUSD>
        
        <ActionButtons>
          <ActionButton
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={() => setShowReceive(false)}
          >
            Send
          </ActionButton>
          <ActionButton
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={handleGenerateAddress}
          >
            Receive
          </ActionButton>
          <ActionButton
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={() => dispatch(updateBalance())}
          >
            Refresh
          </ActionButton>
        </ActionButtons>
      </BalanceCard>

      {!showReceive ? (
        <TransactionSection
          initial={{ y: 50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ delay: 0.4 }}
        >
          <SectionTitle>Send Transaction</SectionTitle>
          
          <FormGroup>
            <Label>Recipient Address</Label>
            <Input
              type="text"
              value={recipient}
              onChange={(e) => setRecipient(e.target.value)}
              placeholder="Enter recipient address..."
            />
          </FormGroup>
          
          <FormGroup>
            <Label>Amount (UNI)</Label>
            <Input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="Enter amount..."
              step="0.000000001"
            />
          </FormGroup>
          
          <ActionButton
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={handleSendTransaction}
            disabled={isLoading}
          >
            {isLoading ? 'Sending...' : 'Send Transaction'}
          </ActionButton>
        </TransactionSection>
      ) : (
        <TransactionSection
          initial={{ y: 50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ delay: 0.4 }}
        >
          <SectionTitle>Receive Unicoin</SectionTitle>
          
          <Label>Your Address</Label>
          <AddressDisplay>
            {address}
            <CopyButton
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              onClick={() => copyToClipboard(address)}
            >
              Copy
            </CopyButton>
          </AddressDisplay>
          
          {newAddress && (
            <>
              <Label>New Address</Label>
              <AddressDisplay>
                {newAddress}
                <CopyButton
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={() => copyToClipboard(newAddress)}
                >
                  Copy
                </CopyButton>
              </AddressDisplay>
            </>
          )}
        </TransactionSection>
      )}

      <TransactionSection
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.6 }}
      >
        <SectionTitle>Recent Transactions</SectionTitle>
        
        <TransactionList>
          {transactions.map((tx, index) => (
            <TransactionItem
              key={index}
              color={tx.type === 'send' ? '#e74c3c' : '#27ae60'}
              initial={{ x: -50, opacity: 0 }}
              animate={{ x: 0, opacity: 1 }}
              transition={{ delay: index * 0.1 }}
            >
              <TransactionDetails>
                <div>
                  <TransactionAmount color={tx.type === 'send' ? '#e74c3c' : '#27ae60'}>
                    {tx.type === 'send' ? '-' : '+'}{formatBalance(tx.amount)} UNI
                  </TransactionAmount>
                  <TransactionInfo>
                    {tx.type === 'send' ? 'To: ' : 'From: '}{tx.address.slice(0, 20)}...
                  </TransactionInfo>
                </div>
                <TransactionInfo>
                  {new Date(tx.timestamp).toLocaleString()}
                </TransactionInfo>
              </TransactionDetails>
            </TransactionItem>
          ))}
        </TransactionList>
      </TransactionSection>
    </WalletContainer>
  );
};

export default Wallet;
