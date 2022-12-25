import {
    Box,
    Flex,
    Heading,
    Icon,
    IconButton,
    List,
    ListItem,
    Menu,
    MenuButton,
    MenuItem,
    MenuList,
    Stack,
    Text,
} from '@chakra-ui/react';
import Icons from '~/services/Icons';
import { ActivityType, GoalType, Habit } from '~/types/types';
import Header from './Header';

const HabitItem = ({
    habit,
    selectHabit,
    selectedHabit,
}: {
    habit: Habit;
    selectedHabit: Habit | null;
    selectHabit: (habit: Habit) => void;
}) => {
    const selected = selectedHabit && habit.id === selectedHabit.id;
    return (
        <Box
            key={habit.id}
            onClick={() => selectHabit(habit)}
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
            <Flex flexDir='column' justifyContent='center'>
                <Text fontSize='lg'>{habit.title}</Text>

                <Text fontSize='sm' color='gray.600'>
                    {habit.activityCounterValue} out of {habit.goal}{' '}
                    {habit.goalType === GoalType.Times ? 'times' : 'minutes'}
                </Text>
            </Flex>

            <Menu>
                <MenuButton
                    as={IconButton}
                    aria-label='Options'
                    icon={<Icon as={Icons.Menu} />}
                    variant='ghost'
                    size='sm'
                />
                <MenuList>
                    <MenuItem>Download</MenuItem>
                    <MenuItem>Create a Copy</MenuItem>
                    <MenuItem>Mark as Draft</MenuItem>
                    <MenuItem>Delete</MenuItem>
                    <MenuItem>Attend a Workshop</MenuItem>
                </MenuList>
            </Menu>
        </Box>
    );
};
const HabitsList = ({
    habits,
    selectHabit,
    selectedHabit,
}: {
    habits: Habit[];
    selectedHabit: Habit | null;
    selectHabit: (habit: Habit) => void;
}) => {
    const completedHabits = habits.filter((habit) => habit.completedToday);
    return (
        <Box borderRightColor='gray.200' borderRightWidth='2px' h='100vh'>
            <Header />
            <Box>
                <Stack spacing={0}>
                    {habits.map((habit) => (
                        <HabitItem
                            key={habit.id}
                            habit={habit}
                            selectHabit={selectHabit}
                            selectedHabit={selectedHabit}
                        />
                    ))}
                </Stack>
                {completedHabits.length > 0 && (
                    <Box mt={4}>
                        <Heading as='h3' size='md' mb={4}>
                            Completed habits
                        </Heading>
                        <List styleType='none'>
                            {completedHabits.map((habit) => (
                                <ListItem key={habit.id} mb={2}>
                                    <Text fontSize='lg'>{habit.title}</Text>
                                    {habit.activityType === ActivityType.Counter && (
                                        <Text fontSize='sm' color='gray.600'>
                                            {habit.activityCounterValue} out of {habit.goal}{' '}
                                            {habit.goalType === GoalType.Times
                                                ? 'times'
                                                : 'minutes'}
                                        </Text>
                                    )}
                                </ListItem>
                            ))}
                        </List>
                    </Box>
                )}
            </Box>
        </Box>
    );
};

export default HabitsList;
