import React from 'react';
import { Box, Button, Heading, Stack, useToast } from '@chakra-ui/react';
import FormField, { FieldsConfig } from '~/common/components/FormField';
import { useForm } from 'react-hook-form';
import validationRules from '~/common/helpers/validationRules';

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
            validationProps: register('currentPassword', validationRules.password()),
        },
        {
            field: 'newPassword',
            label: 'New Password',
            validationProps: register('newPassword', validationRules.newPassword()),
        },
        {
            field: 'newPasswordConfirm',
            label: 'Repeat Password',
            validationProps: register(
                'newPasswordConfirm',
                validationRules.newPasswordConfirm(watch('newPassword')),
            ),
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
