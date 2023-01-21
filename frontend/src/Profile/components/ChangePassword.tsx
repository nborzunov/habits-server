import React from 'react';
import { Box, Button, Heading, Stack, useToast } from '@chakra-ui/react';
import FormField from '~/Profile/components/FormField';
import { useForm } from 'react-hook-form';
import { FieldsConfig } from '~/Profile/types';

const ChangePassword = () => {
    const toast = useToast();
    const initialState = {
        currentPassword: '',
        newPassword: '',
        newPasswordConfirm: '',
    };
    const {
        register,
        watch,
        formState: { errors, isSubmitting },
        handleSubmit,
    } = useForm({
        mode: 'all',
        defaultValues: initialState,
    });

    const onSubmit = (_data: any) => {
        alert('TODO');
    };

    const onError = () => {
        toast({
            title: 'Error',
            description: 'Please check all fields',
            status: 'error',
            isClosable: true,
        });
    };

    const fieldsConfig: FieldsConfig<'currentPassword' | 'newPassword' | 'newPasswordConfirm'> = [
        {
            field: 'currentPassword',
            label: 'Current Password',
            validationProps: register('currentPassword', {
                required: { value: true, message: 'Current password is required' },
                minLength: { value: 8, message: 'Password must be at least 8 characters' },
                // validate: {
                //     value: (value) => value === currentPassword || 'Current password is incorrect',
                // },
            }),
        },
        {
            field: 'newPassword',
            label: 'New Password',
            validationProps: register('newPassword', {
                required: { value: true, message: 'New password is required' },
                minLength: { value: 8, message: 'Password must be at least 8 characters' },
                pattern: {
                    value: /^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$/,
                    message: 'Password must contain at least one letter and one number',
                },
                // validate: (value: string) => {
                //     if (value === currentPassword) {
                //         return 'New password must be different from current password';
                //     }
                // },
            }),
        },
        {
            field: 'newPasswordConfirm',
            label: 'Repeat Password',
            validationProps: register('newPasswordConfirm', {
                required: { value: true, message: 'Confirm new password is required' },
                validate: (value: string) => {
                    if (value !== watch('newPassword')) {
                        return 'Passwords do not match';
                    }
                },
            }),
        },
    ];

    return (
        <Box as={'form'} onSubmit={handleSubmit(onSubmit, onError)}>
            <Heading as='h3' size='md' mb={'6'}>
                Change Password
            </Heading>
            <Stack spacing={4}>
                {fieldsConfig.map(({ field, label, validationProps }) => (
                    <FormField
                        isRequired
                        minWidth={'200px'}
                        key={field}
                        field={field}
                        label={label}
                        validationError={errors[field]}
                        validationProps={validationProps}
                        hideResetButton
                    />
                ))}

                <Stack spacing={10} pt={4}>
                    <Button
                        loadingText='Submitting'
                        size='md'
                        colorScheme={'purple'}
                        width={'160px'}
                        type='submit'
                        isLoading={isSubmitting}
                    >
                        Save changes
                    </Button>
                </Stack>
            </Stack>
        </Box>
    );
};


export default ChangePassword;