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
import { NavLink } from 'react-router-dom';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/services/api';
import { useSetRecoilState } from 'recoil';
import { tokenState } from '~/common/store/atoms';
import useTitle from '~/common/hooks/useTitle';
import { ProfileData } from '~/Profile/types';
import Back from '~/Layout/components/Back';
import Icons from '~/common/services/Icons';

const Signup = ({ refetch }: { refetch: () => void }) => {
    useTitle('Sign Up');
    const setToken = useSetRecoilState(tokenState);
    const [form, setForm] = useState({
        name: '',
        surname: '',
        username: '',
        email: '',
        password: '',
    });
    const [showPassword, setShowPassword] = useState(false);

    const signup = useMutation({
        mutationFn: (data: ProfileData) => {
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
        signup.mutate(form);
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
                <HStack spacing={4}>
                    <FormControl id='name' isRequired>
                        <FormLabel>Name</FormLabel>
                        <Input
                            type='text'
                            focusBorderColor='purple.500'
                            placeholder={'Name'}
                            value={form.name}
                            onChange={(e) => setForm({ ...form, name: e.target.value })}
                        />
                    </FormControl>
                    <FormControl id='surname' isRequired>
                        <FormLabel>Surname</FormLabel>
                        <Input
                            type='text'
                            focusBorderColor='purple.500'
                            placeholder={'Surname'}
                            value={form.surname}
                            onChange={(e) => setForm({ ...form, surname: e.target.value })}
                        />
                    </FormControl>
                </HStack>
                <FormControl id='username' isRequired>
                    <FormLabel>Username</FormLabel>
                    <Input
                        type='text'
                        focusBorderColor='purple.500'
                        placeholder={'Username'}
                        value={form.username}
                        onChange={(e) => setForm({ ...form, username: e.target.value })}
                    />
                </FormControl>

                <FormControl id='email' isRequired>
                    <FormLabel>Email address</FormLabel>
                    <Input
                        type='email'
                        focusBorderColor='purple.500'
                        placeholder={'Email'}
                        value={form.email}
                        onChange={(e) => setForm({ ...form, email: e.target.value })}
                    />
                </FormControl>

                <FormControl id='password' isRequired>
                    <FormLabel>Password</FormLabel>
                    <InputGroup>
                        <Input
                            type={showPassword ? 'text' : 'password'}
                            focusBorderColor='purple.500'
                            placeholder={'Password'}
                            value={form.password}
                            onChange={(e) => setForm({ ...form, password: e.target.value })}
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
                        <NavLink to={'/'}>
                            <Back />
                        </NavLink>

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