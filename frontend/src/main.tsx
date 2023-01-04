import React from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';
import { ChakraProvider, extendTheme } from '@chakra-ui/react';

// styles
import './index.css';
import { RecoilRoot } from 'recoil';

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
            <RecoilRoot>
                <App />
            </RecoilRoot>
        </ChakraProvider>
    </React.StrictMode>,
);
