import {
    Button,
    Checkbox,
    FormControl,
    FormLabel,
    Heading,
    HStack,
    Input,
    Link,
    Stack,
    Text,
} from '@chakra-ui/react';
import React from 'react';
import Back from '~/Layout/components/Back';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/services/api';
import { useSetRecoilState } from 'recoil';
import { tokenState } from '~/common/store/atoms';
import useTitle from '~/common/hooks/useTitle';
import { NavLink } from 'react-router-dom';

const Login = ({ refetch }: { refetch: () => void }) => {
    useTitle('Login');
    const setToken = useSetRecoilState(tokenState);

    const [username, setUsername] = React.useState('');
    const [password, setPassword] = React.useState('');

    const login = useMutation({
        mutationFn: (data: { username: string; password: string }) => {
            return api
                .post<{
                    token: string;
                }>('/auth/', data)
                .then((res) => res.data)
                .then((data) => setToken(`Bearer ${data.token}`))
                .then(() => refetch());
        },
    });
    const handleSubmit = () => {
        // TODO: add form validation
        login.mutate({
            username,
            password,
        });
    };
    return (
        <>
            <Stack align={'center'} pb={8}>
                <Heading fontSize={'4xl'}>Sign in to your account</Heading>
                <Text fontSize={'lg'} color={'gray.600'}>
                    to enjoy all of our cool features ✌️
                </Text>
            </Stack>
            <Stack spacing={4}>
                <FormControl id='username'>
                    <FormLabel>Username</FormLabel>
                    <Input
                        type='text'
                        focusBorderColor='purple.500'
                        placeholder={'Username'}
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                    />
                </FormControl>
                <FormControl id='password'>
                    <FormLabel>Password</FormLabel>
                    <Input
                        type='password'
                        focusBorderColor='purple.500'
                        placeholder={'Password'}
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </FormControl>
                <Stack spacing={10}>
                    <Stack
                        direction={{ base: 'column', sm: 'row' }}
                        align={'start'}
                        justify={'space-between'}
                    >
                        {/*TODO*/}
                        <Checkbox>Remember me</Checkbox>
                        <Link color={'blue.400'}>Forgot password?</Link>
                    </Stack>
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
                            Sign in
                        </Button>
                    </HStack>
                </Stack>
            </Stack>
        </>
    );
};


export default Login;