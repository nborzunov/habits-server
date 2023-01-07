import {
    Box,
    Button,
    Checkbox,
    Flex,
    FormControl,
    FormHelperText,
    FormLabel,
    Heading,
    HStack,
    Icon,
    Input,
    Modal,
    ModalBody,
    ModalCloseButton,
    ModalContent,
    ModalFooter,
    ModalHeader,
    ModalOverlay,
    NumberDecrementStepper,
    NumberIncrementStepper,
    NumberInput,
    NumberInputField,
    NumberInputStepper,
    Select,
    Stack,
    useDisclosure,
} from '@chakra-ui/react';
import Icons from '~/services/Icons';
import { useState } from 'react';
import { GoalType, Habit, HabitData, Periodicity } from '~/types/types';
import { useMutation } from '@tanstack/react-query';
import axios from 'axios';
import { useSetRecoilState } from 'recoil';
import { habitsState } from '~/store/atoms';

const Header = () => {
    const { isOpen, onOpen, onClose } = useDisclosure();

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
            <CreateHabitModal isOpen={isOpen} onClose={onClose} />
        </>
    );
};

const CreateHabitModal = ({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) => {
    const setHabits = useSetRecoilState(habitsState);
    const [title, setTitle] = useState('');
    const [goal, setGoal] = useState(1);
    const [goalType, setGoalType] = useState<GoalType>(GoalType.Times);
    const [periodicity, setPeriodicity] = useState<Periodicity>(Periodicity.Daily);
    const [allowSkip, setAllowSkip] = useState(false);

    const createHabit = useMutation({
        mutationFn: (data: HabitData) =>
            axios
                .post<Habit>('http://localhost:8080/habits', data)
                .then((res) => res.data)
                .then((newHabit) => setHabits((prev) => [newHabit, ...prev]))
                .finally(() => onClose()),
    });
    const handleSubmit = () => {
        createHabit.mutate({
            title,
            goal,
            goalType,
            periodicity,
            allowSkip,
        });
    };
    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>New Habit</ModalHeader>
                <ModalCloseButton />
                <ModalBody>
                    <Stack spacing={3}>
                        <FormControl isRequired>
                            <FormLabel>Title</FormLabel>
                            <Input
                                type='text'
                                value={title}
                                onChange={(e) => setTitle(e.target.value)}
                            />
                        </FormControl>

                        <HStack spacing={3}>
                            <NumberInput
                                allowMouseWheel
                                min={1}
                                value={goal}
                                onChange={(e) => setGoal(Number(e))}
                            >
                                <NumberInputField />
                                <NumberInputStepper>
                                    <NumberIncrementStepper />
                                    <NumberDecrementStepper />
                                </NumberInputStepper>
                            </NumberInput>
                            <Select
                                value={goalType}
                                onChange={(e) => setGoalType(e.target.value as GoalType)}
                            >
                                <option value={GoalType.Times}>Times</option>
                                <option value={GoalType.Mins}>Mins</option>
                            </Select>
                            <Select
                                value={periodicity}
                                onChange={(e) => setPeriodicity(e.target.value as Periodicity)}
                            >
                                <option value={Periodicity.Daily}>Per Day</option>
                                <option value={Periodicity.Weekly}>Per Week</option>
                                <option value={Periodicity.Monthly}>Per Month</option>
                                <option value={Periodicity.Custom}>Custom</option>
                            </Select>
                        </HStack>
                        <FormControl>
                            <FormLabel>Additional</FormLabel>
                            <Box>
                                <Checkbox
                                    isChecked={allowSkip}
                                    onChange={(e) => setAllowSkip(e.target.checked)}
                                >
                                    Allow skip specific days
                                </Checkbox>
                            </Box>
                            <FormHelperText>(e.g. if you need some time to rest)</FormHelperText>
                        </FormControl>
                    </Stack>
                </ModalBody>

                <ModalFooter>
                    <Button colorScheme='blue' mr={3} onClick={onClose}>
                        Close
                    </Button>
                    <Button colorScheme='green' onClick={handleSubmit}>
                        Create
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

export default Header;
