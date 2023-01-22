import { Box, useColorModeValue } from '@chakra-ui/react';
import { Outlet } from 'react-router-dom';

const Auth = () => {
    return (
        <Box
            as='section'
            bg='blue.50'
            _dark={{
                bg: 'gray.700',
            }}
            minH='100vh'
            display={'flex'}
            alignItems={'center'}
            justifyContent={'center'}
        >
            <Box
                rounded={'lg'}
                bg={useColorModeValue('white', 'gray.700')}
                boxShadow={'lg'}
                mx={'auto'}
                maxW={'lg'}
                minW={'500px'}
                p={8}
            >
                <Outlet />
            </Box>
        </Box>
    );
};

export default Auth;
