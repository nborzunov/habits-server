import { Button, Heading, HStack, Link, Stack, Text, useToast } from '@chakra-ui/react';
import React from 'react';
import { NavLink } from 'react-router-dom';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import { useSetRecoilState } from 'recoil';
import { tokenState } from '~/common/store/atoms';
import useTitle from '~/common/hooks/useTitle';
import { ProfileData } from '~/Profile/types';
import Back from '~/Layout/components/Back';
import validationRules from '~/common/helpers/validationRules';
import { useForm } from 'react-hook-form';
import FormField, { FieldsConfig } from '~/common/components/FormField';

const Signup = ({ refetch }: { refetch: () => void }) => {
    useTitle('Sign Up');
    const setToken = useSetRecoilState(tokenState);

    const toast = useToast();
    const signup = useMutation({
        mutationFn: (data: ProfileData) => {
            return api
                .post<{
                    token: string;
                }>('/users/signup', data)
                .then((res) => res.data)
                .then((data) => setToken(`Bearer ${data.token}`))
                .then(() => refetch())
                .then(() =>
                    toast({
                        title: 'Success',
                        description: 'Successfully created account!',
                        status: 'success',
                        duration: 1000,
                        isClosable: true,
                    }),
                )
                .catch((err) =>
                    toast({
                        title: 'Error',
                        description:
                            err.status === 401 ? 'Invalid credentials' : 'Something went wrong',
                        status: 'error',
                        duration: 3000,
                        isClosable: true,
                    }),
                );
        },
    });

    const {
        register,
        watch,
        formState: { errors, isSubmitting },
        handleSubmit,
    } = useForm({
        mode: 'all',
        defaultValues: {
            name: '',
            surname: '',
            username: '',
            email: '',
            password: '',
        },
    });

    const onSubmit = (data: ProfileData) => {
        signup.mutate(data);
    };

    const onError = () => {
        toast({
            title: 'Error',
            description: 'Please check all fields',
            status: 'error',
            isClosable: true,
        });
    };

    const nameFieldsConfig: FieldsConfig<'name' | 'surname'> = [
        {
            field: 'name',
            label: 'Name',
            validationProps: register('name', validationRules.text(3)),
        },
        {
            field: 'surname',
            label: 'Surname',
            validationProps: register('surname', validationRules.text(3)),
        },
    ];
    const otherFieldsConfig: FieldsConfig<'username' | 'email' | 'password'> = [
        {
            field: 'username',
            label: 'Username',
            validationProps: register('username', validationRules.text(6)),
        },
        {
            field: 'email',
            label: 'Email Address',
            validationProps: register('email', validationRules.email()),
        },
        {
            field: 'password',
            label: 'Password',
            validationProps: register('password', validationRules.newPassword()),
        },
    ];

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

            <Stack spacing={4} as={'form'} onSubmit={handleSubmit(onSubmit, onError)}>
                <HStack spacing={4}>
                    {nameFieldsConfig.map(({ field, label, validationProps }) => (
                        <FormField
                            key={field}
                            field={field}
                            label={label}
                            value={watch(field)}
                            validationError={errors[field]}
                            validationProps={validationProps}
                            hideResetButton
                            direction={'column'}
                            variant={'outline'}
                        />
                    ))}
                </HStack>
                {otherFieldsConfig.map(({ field, label, validationProps }) => (
                    <FormField
                        key={field}
                        field={field}
                        label={label}
                        value={watch(field)}
                        validationError={errors[field]}
                        validationProps={validationProps}
                        hideResetButton
                        direction={'column'}
                        variant={'outline'}
                        showPasswordIcon={field === 'password'}
                    />
                ))}

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
                            type={'submit'}
                            isLoading={isSubmitting}
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
