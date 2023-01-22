import { Box, Button, Flex, Heading, HStack, Icon, IconButton, Tooltip } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';
import Statistics from '~/Habits/components/HabitDetails/Statistics';
import {
    TargetCalendarContext,
    YearlyCalendar,
} from '~/Habits/components/HabitDetails/YearlyCalendar';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import TargetChart from '~/Habits/components/HabitDetails/TargetChart';
import { habitsState, selectedHabitState } from '~/common/store/atoms';
import { useMutation } from '@tanstack/react-query';
import api from '~/common/helpers/api';
import MonthlyCalendar from '~/Habits/components/HabitDetails/MonthlyCalendar';
import GridLayout from 'react-grid-layout';
import { useState } from 'react';
import useWidgets, { WidgetIdentifiers } from '~/Habits/hooks/useWidgets';
import Widget from '~/Habits/components/Grid/Widget';
import getCurrentDate from '~/common/utils/getCurrectDate';
import { Habit, TargetType } from '~/Habits/types';

const HabitDetails = () => {
    const habit = useRecoilValue(selectedHabitState);
    const setHabits = useSetRecoilState(habitsState);
    const [isEditMode, setIsEditMode] = useState(false);
    const { save, reset, removeWidget, widgets, props } = useWidgets(isEditMode);

    // console.log(widgets);
    // TODO: вынести мутейшены в отдельный файл
    const createTarget = useMutation({
        mutationFn: (data: {
            id: string | null;
            habitId: string;
            date: Date;
            targetType: string;
        }) => {
            return api
                .post<Habit>('/targets/', data)
                .then((res) => res.data)
                .then((newHabit) =>
                    setHabits((prev) => prev.map((h) => (h.id !== newHabit.id ? h : newHabit))),
                );
        },
    });

    if (!habit) return null;

    const handleCalendarCellClick = (targetId: string | null, date: Date, newType: TargetType) => {
        createTarget.mutate({
            id: targetId,
            habitId: habit.id,
            date: getCurrentDate(date),
            targetType: newType,
        });
    };

    const handleSave = () => {
        save();
        setIsEditMode(false);
    };

    return (
        <Flex width={'100%'}>
            <Box m={0} width={'1600px'}>
                <Flex alignItems={'center'} justifyContent={'space-between'} px={4} pt={2}>
                    <Heading as='h3' size='md'>
                        {habit.title}
                    </Heading>
                    <HStack spacing={2}>
                        {isEditMode && (
                            <HStack spacing={2}>
                                <Tooltip label={'Save'}>
                                    <IconButton
                                        aria-label={'save widgets'}
                                        icon={<Icon as={Icons.Save} />}
                                        onClick={handleSave}
                                        colorScheme={'purple'}
                                    />
                                </Tooltip>

                                <Tooltip label={'Reset'}>
                                    <Button
                                        colorScheme={'purple'}
                                        variant={'outline'}
                                        onClick={reset}
                                    >
                                        Reset
                                    </Button>
                                </Tooltip>

                                <Tooltip label={'Close'}>
                                    <IconButton
                                        aria-label={'close'}
                                        icon={<Icon as={Icons.Cross} />}
                                        onClick={() => setIsEditMode(!isEditMode)}
                                        colorScheme={'purple'}
                                        variant={'outline'}
                                    />
                                </Tooltip>
                            </HStack>
                        )}

                        {!isEditMode && (
                            <Tooltip label={'Edit grid'}>
                                <IconButton
                                    aria-label={'edit grid'}
                                    icon={<Icon as={Icons.Grid} />}
                                    onClick={() => setIsEditMode(!isEditMode)}
                                    colorScheme={'purple'}
                                    variant={'outline'}
                                />
                            </Tooltip>
                        )}
                    </HStack>
                </Flex>
                <Box userSelect={isEditMode ? 'none' : 'auto'}>
                    <GridLayout {...props}>
                        <div key={WidgetIdentifiers.CURRENT_STREAK}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.CURRENT_STREAK)}
                            >
                                <Statistics
                                    title='Current streak'
                                    value={habit.currentStreak}
                                    type='streak'
                                    startDate={habit.currentStreakStartDate}
                                />
                            </Widget>
                        </div>

                        <Box key={WidgetIdentifiers.COMPLETED_CHART}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.COMPLETED_CHART)}
                            >
                                <TargetChart
                                    completed={habit.completedTargets}
                                    failed={habit.failedTargets}
                                />
                            </Widget>
                        </Box>

                        <div key={WidgetIdentifiers.COMPLETED_TARGETS}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.COMPLETED_TARGETS)}
                            >
                                <Statistics
                                    icon={Icons.Complete}
                                    title='Complete'
                                    value={habit.completedTargets}
                                    type='increase'
                                    footerValue={habit.completedTargets}
                                />
                            </Widget>
                        </div>

                        <div key={WidgetIdentifiers.FAILED_TARGETS}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.FAILED_TARGETS)}
                            >
                                <Statistics
                                    icon={Icons.Cross}
                                    title='Failed'
                                    value={habit.failedTargets}
                                    type='decrease'
                                    footerValue={habit.failedTargets}
                                />
                            </Widget>
                        </div>

                        <div key={WidgetIdentifiers.TOTAL_TARGETS}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.TOTAL_TARGETS)}
                            >
                                <Statistics title='Total' value={habit.totalTargets} type='none' />
                            </Widget>
                        </div>

                        <div key={WidgetIdentifiers.SKIPPED_TARGETS}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.SKIPPED_TARGETS)}
                            >
                                <Statistics
                                    icon={Icons.ArrowRight}
                                    title='Skipped'
                                    value={habit.completedTargets}
                                    type='none'
                                />
                            </Widget>
                        </div>

                        <div key={WidgetIdentifiers.YEARLY_CALENDAR}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.YEARLY_CALENDAR)}
                            >
                                <TargetCalendarContext.Provider
                                    value={{
                                        habit,
                                        onCellClick: handleCalendarCellClick,
                                    }}
                                >
                                    <YearlyCalendar targets={habit.targets} />
                                </TargetCalendarContext.Provider>
                            </Widget>
                        </div>
                        <div key={WidgetIdentifiers.MONTHLY_CALENDAR}>
                            <Widget
                                showCross={isEditMode}
                                remove={() => removeWidget(WidgetIdentifiers.MONTHLY_CALENDAR)}
                            >
                                <MonthlyCalendar
                                    targets={habit.targets}
                                    habit={habit}
                                    onCellClick={handleCalendarCellClick}
                                />
                            </Widget>
                        </div>
                    </GridLayout>
                </Box>
            </Box>
            {isEditMode && (
                <Box m={0} flex={'1'}>
                    <Flex alignItems={'center'} justifyContent={'space-between'} px={4} pt={2}>
                        <Heading as='h3' size='md'>
                            Widgets
                        </Heading>
                    </Flex>
                    <HStack spacing={2}>
                        {!widgets.length && <Heading>No widgets left</Heading>}
                        {widgets.map((widget) => (
                            <Box key={widget} p={4}>
                                {widget}
                            </Box>
                        ))}
                    </HStack>
                </Box>
            )}
        </Flex>
    );
};

export default HabitDetails;
