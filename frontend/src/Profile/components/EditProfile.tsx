import { Box, Button, Heading, Stack, useToast } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { EditProfileFields, FieldsConfig, User } from '~/Profile/types';
import EditProfileField from '~/Profile/components/EditProfileField';
import { useForm } from 'react-hook-form';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/services/api';
import { useSetRecoilState } from 'recoil';
import { activeUserState } from '~/common/store/atoms';
import useTitle from '~/common/hooks/useTitle';

type EditProFileData = Required<Pick<User, EditProfileFields>>;

interface Props {
    initialState: EditProFileData;
}

const EditProfile = ({ initialState }: Props) => {
    const [formValues, setFormValues] = useState(initialState);
    const setActiveUser = useSetRecoilState(activeUserState);
    const toast = useToast();

    useTitle(`${initialState.name} ${initialState.surname} - Edit Profile`);

    const {
        register,
        watch,
        formState: { errors, isSubmitting },
        handleSubmit,
        setValue,
    } = useForm({
        mode: 'all',
        defaultValues: initialState,
    });

    const editProfile = useMutation({
        mutationFn: (formData: EditProFileData) => {
            return api
                .put<User>(`/users/me`, formData)
                .then((res) => res.data)
                .then((newUserData) => {
                    setActiveUser(newUserData);
                })
                .then(() =>
                    toast({
                        title: 'Success',
                        description: 'Profile updated!',
                        status: 'success',
                        duration: 1000,
                        isClosable: true,
                    }),
                )
                .catch((err) =>
                    toast({
                        title: 'Error',
                        description: err.message,
                        status: 'error',
                        duration: 3000,
                        isClosable: true,
                    }),
                );
        },
    });

    const onSubmit = (data: EditProFileData) => {
        editProfile.mutate(data);
    };

    const onError = () => {
        toast({
            title: 'Error',
            description: 'Please check all fields',
            status: 'error',
            isClosable: true,
        });
    };

    const fieldsConfig: FieldsConfig = [
        {
            field: 'name',
            label: 'Name',
            validationProps: register('name', {
                required: 'This is required',
                minLength: { value: 3, message: 'Minimum length should be 3' },
            }),
        },
        {
            field: 'surname',
            label: 'Surname',
            validationProps: register('surname', {
                required: 'This is required',
                minLength: { value: 3, message: 'Minimum length should be 3' },
            }),
        },
        {
            field: 'username',
            label: 'Username',
            validationProps: register('username', {
                required: 'This is required',
                minLength: { value: 6, message: 'Minimum length should be 6' },
            }),
        },
        {
            field: 'bio',
            label: 'Profile Bio',
            validationProps: register('bio', {
                maxLength: { value: 250, message: 'Maximum length should be 250' },
            }),
        },
        //     TODO: avatar image
    ];

    useEffect(() => {
        const subscription = watch((value) => {
            setFormValues(value as EditProFileData);
        });
        return () => subscription.unsubscribe();
    }, [watch]);
    return (
        <Box as={'form'} onSubmit={handleSubmit(onSubmit, onError)}>
            <Heading as='h3' size='md' mb={'6'}>
                Edit Profile
            </Heading>
            <Stack spacing={4}>
                {fieldsConfig.map(({ field, label, validationProps }) => (
                    <EditProfileField
                        key={field}
                        field={field}
                        label={label}
                        value={formValues[field]}
                        initialValue={initialState[field]}
                        resetValue={() =>
                            setValue(field, initialState[field], {
                                shouldValidate: true,
                                shouldDirty: true,
                                shouldTouch: true,
                            })
                        }
                        validationError={errors[field]}
                        validationProps={validationProps}
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


export default EditProfile