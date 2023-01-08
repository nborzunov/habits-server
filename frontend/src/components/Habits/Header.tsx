import { Button, Flex, Heading, Icon, useDisclosure } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import { Habit, HabitData } from '~/types/types';
import { useMutation } from '@tanstack/react-query';
import { useSetRecoilState } from 'recoil';
import { habitsState } from '~/store/atoms';
import EditHabitDialog from '~/components/Habits/EditHabitDialog';
import api from '~/services/api';

const Header = () => {
    const { isOpen, onOpen, onClose } = useDisclosure();

    const setHabits = useSetRecoilState(habitsState);

    const createHabit = useMutation({
        mutationFn: (data: HabitData) =>
            api
                .post<Habit>('/habits/', data)
                .then((res) => res.data)
                .then((newHabit) => setHabits((prev) => [newHabit, ...prev]))
                .finally(() => {
                    onClose();
                }),
    });
    const handleSubmit = (formData: HabitData) => {
        createHabit.mutate(formData);
    };

    return (
        <>
            <Flex justifyContent='space-between' alignItems='center' py='8px' px={2}>
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


export default Header;
