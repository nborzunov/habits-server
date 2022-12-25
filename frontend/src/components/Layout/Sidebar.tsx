import { Box, BoxProps, Flex, Text } from '@chakra-ui/react';

import Icons from '~/services/Icons';
import NavItem from '~/components/Layout/NavItem';
import { NavLink } from 'react-router-dom';

const Sidebar = (props: React.PropsWithChildren<BoxProps>) => (
    <Box
        as='nav'
        pos='fixed'
        top='0'
        left='0'
        zIndex='sticky'
        h='full'
        pb='10'
        overflowX='hidden'
        overflowY='auto'
        bg='white'
        w='60'
        {...props}
    >
        <Flex px='8' py='6' align='center'>
            {/* <Logo /> */}
            <Text fontSize='2xl' color='gray.500' fontWeight='semibold'>
                Habits
            </Text>
        </Flex>
        <Flex
            direction='column'
            as='nav'
            fontSize='sm'
            color='gray.500'
            aria-label='Main Navigation'
        >
            <NavLink to='habits'>
                <NavItem icon={Icons.Inbox}>All habits</NavItem>
            </NavLink>
            <NavLink to='dashboard'>
                <NavItem icon={Icons.Dashboard}>Dashboard</NavItem>
            </NavLink>

            {/* <NavItem>Morning</NavItem>
            <NavItem>Collections</NavItem>
            <NavItem>Checklists</NavItem>
            <NavItem>Integrations</NavItem>
            <NavItem>Changelog</NavItem>
            <NavItem>Settings</NavItem> */}
        </Flex>
    </Box>
);

export default Sidebar;
