import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { theme } from '../styles/theme';

export default function ReceiveScreen() {
  return (
    <View style={styles.container}>
      <Text style={styles.text}>Receive Screen - Coming Soon</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: theme.colors.background,
  },
  text: {
    fontSize: theme.fontSize.lg,
    color: theme.colors.text,
  },
});
