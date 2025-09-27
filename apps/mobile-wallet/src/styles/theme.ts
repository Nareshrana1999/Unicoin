export const theme = {
  colors: {
    // Primary colors
    primary: '#3B82F6',
    primaryDark: '#2563EB',
    primaryLight: '#60A5FA',
    
    // Secondary colors
    secondary: '#8B5CF6',
    secondaryDark: '#7C3AED',
    secondaryLight: '#A78BFA',
    
    // Success colors
    success: '#10B981',
    successDark: '#059669',
    successLight: '#34D399',
    
    // Warning colors
    warning: '#F59E0B',
    warningDark: '#D97706',
    warningLight: '#FBBF24',
    
    // Error colors
    error: '#EF4444',
    errorDark: '#DC2626',
    errorLight: '#F87171',
    
    // Neutral colors
    background: '#FFFFFF',
    surface: '#F8FAFC',
    card: '#FFFFFF',
    text: '#1F2937',
    textSecondary: '#6B7280',
    textTertiary: '#9CA3AF',
    border: '#E5E7EB',
    divider: '#F3F4F6',
    gray: '#9CA3AF',
    
    // Dark mode colors
    dark: {
      background: '#0F172A',
      surface: '#1E293B',
      card: '#334155',
      text: '#F1F5F9',
      textSecondary: '#CBD5E1',
      textTertiary: '#94A3B8',
      border: '#475569',
      divider: '#334155',
    },
  },
  
  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 24,
    xl: 32,
    xxl: 48,
  },
  
  borderRadius: {
    sm: 4,
    md: 8,
    lg: 12,
    xl: 16,
    xxl: 24,
    full: 9999,
  },
  
  fontSize: {
    xs: 12,
    sm: 14,
    md: 16,
    lg: 18,
    xl: 20,
    xxl: 24,
    xxxl: 32,
  },
  
  fontWeight: {
    normal: '400' as const,
    medium: '500' as const,
    semibold: '600' as const,
    bold: '700' as const,
    extrabold: '800' as const,
  },
  
  shadows: {
    sm: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 1,
      },
      shadowOpacity: 0.05,
      shadowRadius: 2,
      elevation: 1,
    },
    md: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 2,
      },
      shadowOpacity: 0.1,
      shadowRadius: 4,
      elevation: 3,
    },
    lg: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 4,
      },
      shadowOpacity: 0.15,
      shadowRadius: 8,
      elevation: 5,
    },
  },
  
  gradients: {
    primary: ['#3B82F6', '#8B5CF6'],
    secondary: ['#8B5CF6', '#EC4899'],
    success: ['#10B981', '#059669'],
    warning: ['#F59E0B', '#D97706'],
    error: ['#EF4444', '#DC2626'],
  },
};
