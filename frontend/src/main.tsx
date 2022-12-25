import React from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';
import { ChakraProvider, extendTheme } from '@chakra-ui/react';

// styles
import './index.css';

const theme = extendTheme({
    colors: {
        blue: {
            50: '#F6F8FA',
        },
    },
});

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <ChakraProvider theme={theme}>
            <App />
        </ChakraProvider>
    </React.StrictMode>,
);
