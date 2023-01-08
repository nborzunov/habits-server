import {
    Button,
    FormControl,
    FormLabel,
    Heading,
    HStack,
    Icon,
    Input,
    InputGroup,
    InputRightElement,
    Link,
    Stack,
    Text,
} from '@chakra-ui/react';
import React, { useState } from 'react';
import Icons from '~/services/Icons';
import { NavLink } from 'react-router-dom';
import Back from '~/components/Layout/Back';
import { useMutation } from '@tanstack/react-query';
import api from '~/services/api';
import { useSetRecoilState } from 'recoil';
import { tokenState } from '~/store/atoms';

const Signup = ({ refetch }: { refetch: () => void }) => {
    const setToken = useSetRecoilState(tokenState);
    const [showPassword, setShowPassword] = useState(false);
    const [username, setUsername] = React.useState('');
    const [email, setEmail] = React.useState('');
    const [password, setPassword] = React.useState('');

    const signup = useMutation({
        mutationFn: (data: { username: string; email: string; password: string }) => {
            return api
                .post<{
                    token: string;
                }>('/users/signup', data)
                .then((res) => res.data)
                .then((data) => setToken(`Bearer ${data.token}`))
                .then(() => refetch());
        },
    });
    const handleSubmit = () => {
        // TODO: add form validation
        signup.mutate({
            username,
            email,
            password,
        });
    };

    return (
        <>
            <Stack align={'center'} pb={8}>
                <Heading fontSize={'4xl'} textAlign={'center'}>
                    Sign up
                </Heading>
                <Text fontSize={'lg'} color={'gray.600'}>
                    to enjoy all of our cool features ✌️
                </Text>
            </Stack>
            <Stack spacing={4}>
                <FormControl id='username' isRequired>
                    <FormLabel>Username</FormLabel>
                    <Input
                        type='text'
                        colorScheme={'p'}
                        focusBorderColor='purple.500'
                        placeholder={'Username'}
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                    />
                </FormControl>

                <FormControl id='email' isRequired>
                    <FormLabel>Email address</FormLabel>
                    <Input
                        type='email'
                        focusBorderColor='purple.500'
                        placeholder={'Email'}
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </FormControl>
                <FormControl id='password' isRequired>
                    <FormLabel>Password</FormLabel>
                    <InputGroup>
                        <Input
                            type={showPassword ? 'text' : 'password'}
                            focusBorderColor='purple.500'
                            placeholder={'Password'}
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                        <InputRightElement h={'full'}>
                            <Button
                                variant={'ghost'}
                                onClick={() => setShowPassword((showPassword) => !showPassword)}
                            >
                                {showPassword ? <Icon as={Icons.Show} /> : <Icon as={Icons.Hide} />}
                            </Button>
                        </InputRightElement>
                    </InputGroup>
                </FormControl>
                <Stack spacing={10} pt={4}>
                    <HStack spacing={3}>
                        <Back />

                        <Button
                            loadingText='Submitting'
                            size='lg'
                            colorScheme={'purple'}
                            width={'100%'}
                            onClick={handleSubmit}
                        >
                            Sign up
                        </Button>
                    </HStack>
                </Stack>
                <Stack pt={1}>
                    <Text align={'center'}>
                        Already a user?{' '}
                        <Link as={NavLink} to='/login' color={'blue.400'}>
                            Login
                        </Link>
                    </Text>
                </Stack>
            </Stack>
        </>
    );
};

export default Signup;