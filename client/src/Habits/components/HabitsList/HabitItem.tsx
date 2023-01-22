import { GoalType, Habit, HabitData } from '~/Habits/types';
import { useRecoilState, useSetRecoilState } from 'recoil';
import { habitsState, selectedHabitIdState } from '~/common/store/atoms';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import {
    AlertDialog,
    AlertDialogBody,
    AlertDialogContent,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogOverlay,
    Box,
    Button,
    Flex,
    Icon,
    IconButton,
    Menu,
    MenuButton,
    MenuItem,
    MenuList,
    Text,
    useDisclosure,
} from '@chakra-ui/react';
import React, { useRef } from 'react';
import Icons from '~/common/helpers/Icons';
import EditHabitDialog from '~/Habits/components/EditHabitDialog';
import CompletedCheckbox from '~/Habits/components/HabitsList/CompletedCheckbox';

const HabitItem = ({ habit }: { habit: Habit }) => {
    const [selectedHabitId, setSelectedHabitId] = useRecoilState(selectedHabitIdState);
    const setHabits = useSetRecoilState(habitsState);
    const selected = selectedHabitId && habit.id === selectedHabitId;
    const completed = habit.completedToday;

    const editHabit = useMutation({
        mutationFn: (formData: HabitData) => {
            return api
                .put<Habit>(`/habits/${habit.id}`, formData)
                .then((res) => res.data)
                .then((newHabit) => {
                    setHabits((prev) => prev.map((h) => (h.id === habit.id ? newHabit : h)));
                })
                .finally(() => onCloseEditHabit());
        },
    });

    const deleteHabit = useMutation({
        mutationFn: () => {
            return api
                .delete<Habit>(`/habits/${habit.id}`)
                .then((res) => res.data)
                .then(() => {
                    setHabits((prev) => prev.filter((h) => h.id !== habit.id));
                    if (selectedHabitId === habit.id) {
                        setSelectedHabitId(null);
                    }
                })
                .finally(() => onCloseDeleteConfirm());
        },
    });

    const archiveHabit = useMutation({
        mutationFn: () => {
            return api
                .put<Habit>(`/habits/${habit.id}/archive/`)
                .then((res) => res.data)
                .then(() => {
                    setHabits((prev) => prev.filter((h) => h.id !== habit.id));
                    if (selectedHabitId === habit.id) {
                        setSelectedHabitId(null);
                    }
                })
                .finally(() => onCloseDeleteConfirm());
        },
    });

    const handleEdit = (h: HabitData) => {
        editHabit.mutate(h);
    };

    const handleDelete = () => {
        deleteHabit.mutate();
    };
    const handleArchive = () => {
        archiveHabit.mutate();
    };

    const {
        isOpen: isOpenDeleteConfirm,
        onOpen: onOpenDeleteConfirm,
        onClose: onCloseDeleteConfirm,
    } = useDisclosure();
    const cancelRef = useRef();

    const {
        isOpen: isOpenEditHabit,
        onOpen: onOpenEditHabit,
        onClose: onCloseEditHabit,
    } = useDisclosure();

    return (
        <>
            <Box
                key={habit.id}
                onClick={() => setSelectedHabitId(habit.id)}
                bg={selected ? 'blackAlpha.50' : 'transparent'}
                transition='all 0.2s ease'
                _hover={{
                    bg: selected ? 'blackAlpha.200' : 'blackAlpha.50',
                    cursor: 'pointer',
                }}
                p={2}
                px={4}
                h='64px'
                display='flex'
                justifyContent='space-between'
                alignItems='center'
            >
                <Flex alignItems={'center'} justifyContent={'center'}>
                    <CompletedCheckbox value={completed} habit={habit}></CompletedCheckbox>
                    <Flex flexDir='column' justifyContent='center'>
                        <Text fontSize='lg'>{habit.title}</Text>

                        <Text fontSize='sm' color='gray.600'>
                            {habit.goal} {habit.goalType === GoalType.Times ? 'times' : 'minutes'}
                        </Text>
                    </Flex>
                </Flex>
                <Menu>
                    <MenuButton
                        as={IconButton}
                        aria-label='Options'
                        icon={<Icon as={Icons.Menu} />}
                        variant='ghost'
                        size='sm'
                        onClick={(e) => e.stopPropagation()}
                    />
                    <MenuList p={0}>
                        <MenuItem
                            onClick={onOpenEditHabit}
                            pl='4'
                            rounded='md'
                            py='3'
                            cursor='pointer'
                            color='gray.600'
                            _hover={{
                                bg: 'purple.300',
                                color: 'whiteAlpha.900',
                            }}
                            role='group'
                            fontWeight='semibold'
                            transition='.15s ease'
                            onMouseOver={(e) => e.stopPropagation()}
                        >
                            <Flex alignItems={'center'} align='center'>
                                <Icon as={Icons.Edit} mr={2} />
                                <Text>Edit</Text>
                            </Flex>
                        </MenuItem>
                        <MenuItem
                            onClick={onOpenDeleteConfirm}
                            pl='4'
                            rounded='md'
                            py='3'
                            cursor='pointer'
                            color='gray.600'
                            _hover={{
                                bg: 'purple.300',
                                color: 'whiteAlpha.900',
                            }}
                            role='group'
                            fontWeight='semibold'
                            transition='.15s ease'
                        >
                            <Flex alignItems={'center'}>
                                <Icon as={Icons.Delete} mr={2} />
                                <Text>Delete</Text>
                            </Flex>
                        </MenuItem>
                        <MenuItem
                            onClick={handleArchive}
                            pl='4'
                            rounded='md'
                            py='3'
                            cursor='pointer'
                            color='gray.600'
                            _hover={{
                                bg: 'purple.300',
                                color: 'whiteAlpha.900',
                            }}
                            role='group'
                            fontWeight='semibold'
                            transition='.15s ease'
                        >
                            <Flex alignItems={'center'}>
                                <Icon as={Icons.Archive} mr={2} />
                                <Text>Archive</Text>
                            </Flex>
                        </MenuItem>
                    </MenuList>
                </Menu>
            </Box>
            <AlertDialog
                isOpen={isOpenDeleteConfirm}
                onClose={onCloseDeleteConfirm}
                leastDestructiveRef={cancelRef as any}
            >
                <AlertDialogOverlay>
                    <AlertDialogContent>
                        <AlertDialogHeader fontSize='lg' fontWeight='bold'>
                            Delete Habit "{habit.title}"
                        </AlertDialogHeader>

                        <AlertDialogBody>
                            Are you sure? If you delete this habit, you will lose all your progress.
                        </AlertDialogBody>

                        <AlertDialogFooter>
                            <Button onClick={onCloseDeleteConfirm}>Cancel</Button>
                            <Button colorScheme='blue' onClick={handleArchive} ml={3}>
                                Archive
                            </Button>
                            <Button colorScheme='red' onClick={handleDelete} ml={3}>
                                Delete
                            </Button>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialogOverlay>
            </AlertDialog>

            <EditHabitDialog
                isOpen={isOpenEditHabit}
                onClose={onCloseEditHabit}
                onSubmit={handleEdit}
                initialState={habit}
            />
        </>
    );
};

export default HabitItem;
