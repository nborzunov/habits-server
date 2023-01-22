import { Box, BoxProps, Flex, Text } from '@chakra-ui/react';

import Icons from '~/common/helpers/Icons';
import NavItem from '~/Layout/components/NavItem';
import { NavLink } from 'react-router-dom';
import React from 'react';
import ProfileInfo from '~/Profile/components/ProfileInfo';

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
            // my={'16'}
        >
            <NavLink to='me'>
                <ProfileInfo />
            </NavLink>
            <NavLink to='habits'>
                <NavItem icon={Icons.Inbox}>All habits</NavItem>
            </NavLink>
            <NavLink to='dashboard'>
                <NavItem icon={Icons.Dashboard}>Dashboard</NavItem>
            </NavLink>
        </Flex>

        {/*<Flex*/}
        {/*    direction='column'*/}
        {/*    as='nav'*/}
        {/*    fontSize='sm'*/}
        {/*    color='gray.500'*/}
        {/*    aria-label='Main Navigation'*/}
        {/*>*/}
        {/*    <NavLink to='habits'>*/}
        {/*        <NavItem icon={Icons.Inbox}>All habits</NavItem>*/}
        {/*    </NavLink>*/}

        {/*    <NavItem>Logout</NavItem>*/}
        {/*</Flex>*/}
    </Box>
);

export default Sidebar;
