


   function process_audio()
    % Vibrato function
    function y = vibrato(x, SAMPLERATE, Modfreq, Width)
        % Author: S. Disch
        %
        %--------------------------------------------------------------------------
        % This source code is provided without any warranties as published in 
        % DAFX book 2nd edition, copyright Wiley & Sons 2011, available at 
        % http://www.dafx.de. It may be used for educational purposes and not 
        % for commercial applications without further permission.
        %--------------------------------------------------------------------------
        
        ya_alt=0;
        Delay=Width; % basic delay of input sample in sec
        DELAY=round(Delay*SAMPLERATE); % basic delay in # samples
        WIDTH=round(Width*SAMPLERATE); % modulation width in # samples
        if WIDTH>DELAY 
          error('delay greater than basic delay !!!');
          return;
        end
        MODFREQ=Modfreq/SAMPLERATE; % modulation frequency in # samples
        LEN=length(x);        % # of samples in WAV-file
        L=2+DELAY+WIDTH*2;    % length of the entire delay  
        Delayline=zeros(L,1); % memory allocation for delay
        y=zeros(size(x));     % memory allocation for output vector
        for n=1:(LEN-1)
           M=MODFREQ;
           MOD=sin(M*2*pi*n);
           TAP=1+DELAY+WIDTH*MOD;
           i=floor(TAP);
           frac=TAP-i;
           Delayline=[x(n);Delayline(1:L-1)]; 
           %---Linear Interpolation-----------------------------
           y(n,1)=Delayline(i+1)*frac+Delayline(i)*(1-frac);
           %---Allpass Interpolation------------------------------
           %y(n,1)=(Delayline(i+1)+(1-frac)*Delayline(i)-(1-frac)*ya_alt);  
           %ya_alt=ya(n,1);
           %---Spline Interpolation-------------------------------
           %y(n,1)=Delayline(i+1)*frac^3/6
           %....+Delayline(i)*((1+frac)^3-4*frac^3)/6
           %....+Delayline(i-1)*((2-frac)^3-4*(1-frac)^3)/6
           %....+Delayline(i-2)*(1-frac)^3/6; 
           %3rd-order Spline Interpolation
        end  
    end

    % Function to normalize data to the range of reference data
    function norm_data = normalize_data(data, reference)
        data_min = min(data);
        data_max = max(data);
        ref_min = min(reference);
        ref_max = max(reference);

        norm_data = ((data - data_min) / (data_max - data_min)) * (ref_max - ref_min) + ref_min;
    end

    % Read audio files
    [x1, fs1] = audioread('DrumLoop1.wav');
    [x2, fs2] = audioread('DrumLoop2.wav');
    [x3, fs3] = audioread('SynthArp.wav');

    % Read text files
    data1 = load('DrumLoop1.txt');
    data2 = load('DrumLoop2.txt');
    data3 = load('SynthArp.txt');


    % Parameters for vibrato effect
    Modfreq = 5; % Modulation frequency in Hz
    Width = 0.005; % Width of modulation in seconds

    % Process audio files through vibrato
    y1 = vibrato(x1, fs1, Modfreq, Width);
    y2 = vibrato(x2, fs2, Modfreq, Width);
    y3 = vibrato(x3, fs3, Modfreq, Width);

    % Normalize comparison data to the range of processed audio data
    norm_data1 = normalize_data(data1, y1);
    norm_data2 = normalize_data(data2, y2);
    norm_data3 = normalize_data(data3, y3);

    % Plot the original and processed audio signals
    figure;
    subplot(3,2,1);
    plot(x1);
    title('Original Audio 1');
    subplot(3,2,2);
    plot(y1);
    title('Processed Audio 1');

    subplot(3,2,3);
    plot(x2);
    title('Original Audio 2');
    subplot(3,2,4);
    plot(y2);
    title('Processed Audio 2');

    subplot(3,2,5);
    plot(x3);
    title('Original Audio 3');
    subplot(3,2,6);
    plot(y3);
    title('Processed Audio 3');

    % Compare processed audio with normalized text file samples
    figure;
    subplot(3,1,1);
    plot(y1(1:length(norm_data1)), 'r');
    hold on;
    plot(norm_data1, 'b');
    title('Comparison of Matlab Vibrato and Rust Vibrato 1');
    legend('MatLab', 'Rust');

    subplot(3,1,2);
    plot(y2(1:length(norm_data2)), 'r');
    hold on;
    plot(norm_data2, 'b');
    title('Comparison of Matlab Vibrato and Rust Vibrato 2');
    legend('MatLab', 'Rust');

    subplot(3,1,3);
    plot(y3(1:length(norm_data3)), 'r');
    hold on;
    plot(norm_data3, 'b');
    title('Comparison of Matlab Vibrato and Rust Vibrato 3');
    legend('MatLab', 'Rust');
end
