import { Button, Grid, GridItem, Heading, Stack, Text } from '@chakra-ui/react';
import React from 'react';
import { NavLink } from 'react-router-dom';
import useTitle from '~/common/hooks/useTitle';

const AuthStartup = () => {
    useTitle('Home');
    return (
        <>
            <Stack align={'center'} pb={8}>
                <Heading fontSize={'4xl'}>Welcome to Habits</Heading>
                <Text fontSize={'lg'} color={'gray.600'}>
                    Sign in to your account and start building good habits.
                </Text>
            </Stack>
            <Grid templateColumns={'repeat(2, 1fr)'} gap={4} mt={2}>
                <GridItem>
                    <NavLink to='login'>
                        <Button
                            loadingText='Submitting'
                            size='lg'
                            colorScheme={'purple'}
                            variant={'outline'}
                            width={'100%'}
                        >
                            Sign in
                        </Button>
                    </NavLink>
                </GridItem>
                <GridItem>
                    <NavLink to='signup'>
                        <Button
                            loadingText='Submitting'
                            size='lg'
                            colorScheme={'purple'}
                            width={'100%'}
                        >
                            Sign up
                        </Button>
                    </NavLink>
                </GridItem>
            </Grid>
        </>
    );
};

export default AuthStartup;
