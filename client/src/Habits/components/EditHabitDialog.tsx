import { useState } from 'react';
import {
    Box,
    Button,
    Checkbox,
    FormControl,
    FormHelperText,
    FormLabel,
    HStack,
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
} from '@chakra-ui/react';
import { GoalType, HabitData, Periodicity } from '~/Habits/types';

const EditHabitDialog = ({
    isOpen,
    onClose,
    initialState,
    onSubmit,
}: {
    isOpen: boolean;
    onClose: () => void;
    initialState?: HabitData;
    onSubmit: (h: HabitData) => void;
}) => {
    const [title, setTitle] = useState(initialState?.title ?? '');
    const [goal, setGoal] = useState(initialState?.goal ?? 1);
    const [goalType, setGoalType] = useState<GoalType>(initialState?.goalType ?? GoalType.Times);
    const [periodicity, setPeriodicity] = useState<Periodicity>(
        initialState?.periodicity ?? Periodicity.Daily,
    );
    const [allowSkip, setAllowSkip] = useState(initialState?.allowSkip ?? false);

    const handleSubmit = () => {
        onSubmit({
            title,
            goal,
            goalType,
            periodicity,
            allowSkip,
        });
        clearForm();
    };
    const clearForm = () => {
        setTitle('');
        setGoal(1);
        setGoalType(GoalType.Times);
        setPeriodicity(Periodicity.Daily);
        setAllowSkip(false);
    };

    const handleChangeTitle = (value: string) => {
        setTitle(value.charAt(0).toUpperCase() + value.slice(1));
    };

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>
                    {initialState ? `Edit Habit "${initialState.title}"` : 'New Habit'}
                </ModalHeader>
                <ModalCloseButton />
                <ModalBody>
                    <Stack spacing={3}>
                        <FormControl isRequired>
                            <FormLabel>Title</FormLabel>
                            <Input
                                type='text'
                                value={title}
                                onChange={(e) => handleChangeTitle(e.target.value)}
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
                        {initialState ? 'Update' : 'Create'}
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

export default EditHabitDialog;
