#[doc = "Register `OUT_SET` reader"]
pub struct R(crate::R<OUT_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_SET` writer"]
pub struct W(crate::W<OUT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE` reader - Data delay in serial RGB format (based on OUTCLK)"]
pub type PHASE_R = crate::FieldReader<u8, PHASE_A>;
#[doc = "Data delay in serial RGB format (based on OUTCLK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHASE_A {
    #[doc = "3: 3 cycles"]
    _11 = 3,
    #[doc = "2: 2 cycles"]
    _10 = 2,
    #[doc = "1: 1 cycle"]
    _01 = 1,
    #[doc = "0: 0 cycle"]
    _00 = 0,
}
impl From<PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        variant as _
    }
}
impl PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_A {
        match self.bits {
            3 => PHASE_A::_11,
            2 => PHASE_A::_10,
            1 => PHASE_A::_01,
            0 => PHASE_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PHASE_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PHASE_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PHASE_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PHASE_A::_00
    }
}
#[doc = "Field `PHASE` writer - Data delay in serial RGB format (based on OUTCLK)"]
pub type PHASE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUT_SET_SPEC, u8, PHASE_A, 2, O>;
impl<'a, const O: u8> PHASE_W<'a, O> {
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PHASE_A::_11)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PHASE_A::_10)
    }
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PHASE_A::_01)
    }
    #[doc = "0 cycle"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PHASE_A::_00)
    }
}
#[doc = "Field `DIRSEL` reader - Invalid data position control in serial RGB format"]
pub type DIRSEL_R = crate::BitReader<DIRSEL_A>;
#[doc = "Invalid data position control in serial RGB format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSEL_A {
    #[doc = "1: Invalid data is output prior to valid (RGB) data."]
    _1 = 1,
    #[doc = "0: Invalid data is output following valid (RGB) data."]
    _0 = 0,
}
impl From<DIRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSEL_A {
        match self.bits {
            true => DIRSEL_A::_1,
            false => DIRSEL_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRSEL_A::_0
    }
}
#[doc = "Field `DIRSEL` writer - Invalid data position control in serial RGB format"]
pub type DIRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_SET_SPEC, DIRSEL_A, O>;
impl<'a, const O: u8> DIRSEL_W<'a, O> {
    #[doc = "Invalid data is output prior to valid (RGB) data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRSEL_A::_1)
    }
    #[doc = "Invalid data is output following valid (RGB) data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRSEL_A::_0)
    }
}
#[doc = "Field `FRQSEL` reader - Clock frequency division control"]
pub type FRQSEL_R = crate::FieldReader<u8, FRQSEL_A>;
#[doc = "Clock frequency division control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQSEL_A {
    #[doc = "3: Setting prohibited"]
    _11 = 3,
    #[doc = "2: Quarter frequency (serial RGB)"]
    _10 = 2,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "0: No frequency division, parallel RGB"]
    _00 = 0,
}
impl From<FRQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQSEL_A) -> Self {
        variant as _
    }
}
impl FRQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQSEL_A {
        match self.bits {
            3 => FRQSEL_A::_11,
            2 => FRQSEL_A::_10,
            1 => FRQSEL_A::_01,
            0 => FRQSEL_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FRQSEL_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FRQSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FRQSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FRQSEL_A::_00
    }
}
#[doc = "Field `FRQSEL` writer - Clock frequency division control"]
pub type FRQSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUT_SET_SPEC, u8, FRQSEL_A, 2, O>;
impl<'a, const O: u8> FRQSEL_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FRQSEL_A::_11)
    }
    #[doc = "Quarter frequency (serial RGB)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FRQSEL_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FRQSEL_A::_01)
    }
    #[doc = "No frequency division, parallel RGB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FRQSEL_A::_00)
    }
}
#[doc = "Field `FORMAT` reader - Output format select"]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Output format select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "3: Serial RGB; select RGB888 as dither output format."]
    _11 = 3,
    #[doc = "2: RGB565; select RGB565 as dither output format."]
    _10 = 2,
    #[doc = "1: RGB666; select RGB666 as dither output format."]
    _01 = 1,
    #[doc = "0: RGB888; select RGB888 as dither output format."]
    _00 = 0,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            3 => FORMAT_A::_11,
            2 => FORMAT_A::_10,
            1 => FORMAT_A::_01,
            0 => FORMAT_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FORMAT_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FORMAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FORMAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FORMAT_A::_00
    }
}
#[doc = "Field `FORMAT` writer - Output format select"]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUT_SET_SPEC, u8, FORMAT_A, 2, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "Serial RGB; select RGB888 as dither output format."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FORMAT_A::_11)
    }
    #[doc = "RGB565; select RGB565 as dither output format."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FORMAT_A::_10)
    }
    #[doc = "RGB666; select RGB666 as dither output format."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FORMAT_A::_01)
    }
    #[doc = "RGB888; select RGB888 as dither output format."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FORMAT_A::_00)
    }
}
#[doc = "Field `SWAPON` reader - Pixel order control"]
pub type SWAPON_R = crate::BitReader<SWAPON_A>;
#[doc = "Pixel order control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAPON_A {
    #[doc = "1: In the order of BGR"]
    _1 = 1,
    #[doc = "0: In the order of RGB"]
    _0 = 0,
}
impl From<SWAPON_A> for bool {
    #[inline(always)]
    fn from(variant: SWAPON_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAPON_A {
        match self.bits {
            true => SWAPON_A::_1,
            false => SWAPON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAPON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAPON_A::_0
    }
}
#[doc = "Field `SWAPON` writer - Pixel order control"]
pub type SWAPON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_SET_SPEC, SWAPON_A, O>;
impl<'a, const O: u8> SWAPON_W<'a, O> {
    #[doc = "In the order of BGR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAPON_A::_1)
    }
    #[doc = "In the order of RGB"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAPON_A::_0)
    }
}
#[doc = "Field `ENDIANON` reader - Bit endian change control"]
pub type ENDIANON_R = crate::BitReader<ENDIANON_A>;
#[doc = "Bit endian change control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANON_A {
    #[doc = "1: Ascending order (big endian)"]
    _1 = 1,
    #[doc = "0: Descending order (little endian)"]
    _0 = 0,
}
impl From<ENDIANON_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANON_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIANON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANON_A {
        match self.bits {
            true => ENDIANON_A::_1,
            false => ENDIANON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDIANON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDIANON_A::_0
    }
}
#[doc = "Field `ENDIANON` writer - Bit endian change control"]
pub type ENDIANON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_SET_SPEC, ENDIANON_A, O>;
impl<'a, const O: u8> ENDIANON_W<'a, O> {
    #[doc = "Ascending order (big endian)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDIANON_A::_1)
    }
    #[doc = "Descending order (little endian)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDIANON_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data delay in serial RGB format (based on OUTCLK)"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Invalid data position control in serial RGB format"]
    #[inline(always)]
    pub fn dirsel(&self) -> DIRSEL_R {
        DIRSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock frequency division control"]
    #[inline(always)]
    pub fn frqsel(&self) -> FRQSEL_R {
        FRQSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Output format select"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 24 - Pixel order control"]
    #[inline(always)]
    pub fn swapon(&self) -> SWAPON_R {
        SWAPON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit endian change control"]
    #[inline(always)]
    pub fn endianon(&self) -> ENDIANON_R {
        ENDIANON_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data delay in serial RGB format (based on OUTCLK)"]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<0> {
        PHASE_W::new(self)
    }
    #[doc = "Bit 4 - Invalid data position control in serial RGB format"]
    #[inline(always)]
    #[must_use]
    pub fn dirsel(&mut self) -> DIRSEL_W<4> {
        DIRSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock frequency division control"]
    #[inline(always)]
    #[must_use]
    pub fn frqsel(&mut self) -> FRQSEL_W<8> {
        FRQSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Output format select"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<12> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 24 - Pixel order control"]
    #[inline(always)]
    #[must_use]
    pub fn swapon(&mut self) -> SWAPON_W<24> {
        SWAPON_W::new(self)
    }
    #[doc = "Bit 28 - Bit endian change control"]
    #[inline(always)]
    #[must_use]
    pub fn endianon(&mut self) -> ENDIANON_W<28> {
        ENDIANON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Block Output Interface Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_set](index.html) module"]
pub struct OUT_SET_SPEC;
impl crate::RegisterSpec for OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_set::R](R) reader structure"]
impl crate::Readable for OUT_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_set::W](W) writer structure"]
impl crate::Writable for OUT_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OUT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
