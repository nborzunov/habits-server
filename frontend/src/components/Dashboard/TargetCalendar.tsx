import { Box, Flex, Grid, GridItem, Text, Tooltip, useTheme } from '@chakra-ui/react';
import dayjs from 'dayjs';
import { Habit, Target, TargetType } from '~/types/types';
import getLoop from '~/utils/getLoop';

const Cell = ({
    dayId,
    monthId,
    daysInMonth,
    size,
    targetsMap,
    habit,
    onClick,
}: {
    dayId: number;
    monthId: number;
    daysInMonth: number;
    size: number;
    targetsMap: Record<string, Target>;
    habit: Habit;
    onClick: (targetId: string | null, date: Date, newType: TargetType) => void;
}) => {
    const sizePx = `${size}px`;
    const theme = useTheme();

    if (dayId < 0 || dayId >= daysInMonth) {
        return <Box key={'empty' + monthId + dayId} width={sizePx} height={sizePx} />;
    }

    const green = theme.colors.green;

    const day = dayjs(`2023-${monthId + 1}-${dayId + 2}`);

    const target = targetsMap[day.format('DD/MM/YYYY')];

    const handleClick = () => {
        let newType;

        if (!target) {
            newType = TargetType.Done;
        } else if (target.targetType === TargetType.Done && habit.allowSkip) {
            newType = TargetType.Skip;
        } else {
            newType = TargetType.Empty;
        }

        onClick(target?.id, day.toDate(), newType);
    };
    return (
        <Box key={monthId + dayId} cursor='pointer'>
            <Tooltip label={dayjs(`2023-${monthId + 1}-${dayId + 1}`).format('D MMMM YYYY')}>
                {target && target.targetType === TargetType.Skip ? (
                    <Box
                        width={sizePx}
                        height={sizePx}
                        bg={'gray.300'}
                        borderTop='10px solid transparent'
                        borderWidth={`${sizePx} 0 0 ${sizePx}`}
                        borderColor={`transparent transparent transparent ${green[500]}}`}
                        onClick={handleClick}
                    ></Box>
                ) : (
                    <Box
                        width={sizePx}
                        height={sizePx}
                        bg={target ? 'green.500' : 'gray.300'}
                        onClick={handleClick}
                    ></Box>
                )}
            </Tooltip>
        </Box>
    );
};

const Month = ({
    monthId,
    size,
    targetsMap,
    habit,
    onClick,
}: {
    monthId: number;
    size: number;
    targetsMap: Record<string, Target>;
    habit: Habit;
    onClick: (targetId: string | null, date: Date, newType: TargetType) => void;
}) => {
    const daysInMonth = dayjs(`2023-${monthId + 1}-1`).daysInMonth();
    const firstDay = dayjs(`2023-${monthId + 1}-1`).day();
    const columns = Math.ceil((firstDay + daysInMonth) / 7);

    const gaps = [3, 3];
    const cellSizes = [12, 12];
    const gap = gaps[size];
    const cellSize = cellSizes[size];

    const month = dayjs(`2023-${monthId + 1}-1`).format('MMM');
    return (
        <Box p='1'>
            <Text pb='1' textAlign='center' fontWeight='bold'>
                {month}
            </Text>
            <Grid templateColumns={`repeat(${columns}, ${cellSize}px)`} gap={`${gap}px`}>
                {getLoop(columns).map((columnId) => (
                    <GridItem key={monthId + columnId}>
                        <Grid templateRows={`repeat(7, ${cellSize}px)`} gap={`${gap}px`}>
                            {getLoop(7).map((rowId) => (
                                <Cell
                                    key={monthId + columnId + rowId}
                                    monthId={monthId}
                                    dayId={columnId * 7 + rowId - firstDay}
                                    daysInMonth={daysInMonth}
                                    size={cellSize}
                                    targetsMap={targetsMap}
                                    habit={habit}
                                    onClick={onClick}
                                />
                            ))}
                        </Grid>
                    </GridItem>
                ))}
            </Grid>
        </Box>
    );
};
const TargetCalendar = ({
    size,
    targets,
    habit,
    onCellClick,
}: {
    size?: 'sm' | 'md';
    targets: Target[];
    habit: Habit;
    onCellClick: (targetId: string | null, date: Date, newType: TargetType) => void;
}) => {
    const targetsMap = targets.reduce((acc, target) => {
        acc[dayjs(target.date).format('DD/MM/YYYY')] = target;
        return acc;
    }, {} as { [key: string]: Target });
    size = size || 'md';
    return (
        <Box>
            <Flex p='2'>
                {getLoop(12).map((i) => (
                    <Month
                        key={i}
                        monthId={i}
                        size={size === 'sm' ? 0 : 1}
                        targetsMap={targetsMap}
                        habit={habit}
                        onClick={onCellClick}
                    />
                ))}
            </Flex>
        </Box>
    );
};

export default TargetCalendar;
