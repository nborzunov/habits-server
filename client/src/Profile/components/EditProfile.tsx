import { Box, Button, Heading, Stack, useToast } from '@chakra-ui/react';
import { EditProfileFields, User } from '~/Profile/types';
import { useMutation } from '@tanstack/react-query';
import { useForm } from 'react-hook-form';
import { activeUserState } from '~/common/store/atoms';
import useTitle from '~/common/hooks/useTitle';
import React from 'react';
import api from '~/common/helpers/api';
import { useSetRecoilState } from 'recoil';
import FormField, { FieldsConfig } from '~/common/components/FormField';
import validationRules from '~/common/helpers/validationRules';

type EditProFileData = Required<Pick<User, EditProfileFields>>;

interface Props {
    initialState: EditProFileData;
}

const EditProfile = ({ initialState }: Props) => {
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

    const fieldsConfig: FieldsConfig<'name' | 'surname' | 'username' | 'bio'> = [
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
        {
            field: 'username',
            label: 'Username',
            validationProps: register('username', validationRules.text(6)),
        },
        {
            field: 'bio',
            label: 'Profile Bio',
            validationProps: register('bio', validationRules.longText()),
        },
        //     TODO: avatar image
    ];

    return (
        <Box as={'form'} onSubmit={handleSubmit(onSubmit, onError)}>
            <Heading as='h3' size='md' mb={'6'}>
                Edit Profile
            </Heading>
            <Stack spacing={4}>
                {fieldsConfig.map(({ field, label, validationProps }) => (
                    <FormField
                        key={field}
                        field={field}
                        label={label}
                        value={watch(field)}
                        initialValue={initialState[field]}
                        resetValue={() =>
                            setValue(field, initialState[field], {
                                shouldValidate: true,
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

export default EditProfile;
