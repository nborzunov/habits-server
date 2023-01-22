import { Button, Flex, Heading, Icon, useDisclosure, useToast } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';
import { useMutation } from '@tanstack/react-query';
import { useSetRecoilState } from 'recoil';
import { habitsState } from '~/common/store/atoms';
import EditHabitDialog from '~/Habits/components/EditHabitDialog';
import api from '~/common/helpers/api';
import { Habit, HabitData } from '~/Habits/types';

const HabitsListHeader = () => {
    const { isOpen, onOpen, onClose } = useDisclosure();

    const setHabits = useSetRecoilState(habitsState);

    const toast = useToast();

    const createHabit = useMutation({
        mutationFn: (data: HabitData) =>
            api
                .post<Habit>('/habits/', data)
                .then((res) => res.data)
                .then((newHabit) => setHabits((prev) => [newHabit, ...prev]))
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
                )
                .finally(() => {
                    onClose();
                }),
    });
    const handleSubmit = (formData: HabitData) => {
        createHabit.mutate(formData);
    };

    return (
        <>
            <Flex justifyContent='space-between' alignItems='center' p={2}>
                <Heading as='h3' size='md' mb={'12px'}>
                    All habits
                </Heading>
                <Button colorScheme='blue' variant='solid' size='sm' onClick={onOpen}>
                    <Icon as={Icons.Add} fontSize={'20px'} /> Add Habits
                </Button>
            </Flex>
            <EditHabitDialog onSubmit={handleSubmit} isOpen={isOpen} onClose={onClose} />
        </>
    );
};

export default HabitsListHeader;
