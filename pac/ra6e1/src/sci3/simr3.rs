#[doc = "Register `SIMR3` reader"]
pub struct R(crate::R<SIMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIMR3` writer"]
pub struct W(crate::W<SIMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIMR3_SPEC>;
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
impl From<crate::W<SIMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICSTAREQ` reader - Start Condition Generation"]
pub type IICSTAREQ_R = crate::BitReader<IICSTAREQ_A>;
#[doc = "Start Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTAREQ_A {
    #[doc = "0: Do not generate start condition"]
    _0 = 0,
    #[doc = "1: Generate start condition"]
    _1 = 1,
}
impl From<IICSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTAREQ_A {
        match self.bits {
            false => IICSTAREQ_A::_0,
            true => IICSTAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTAREQ_A::_1
    }
}
#[doc = "Field `IICSTAREQ` writer - Start Condition Generation"]
pub type IICSTAREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, SIMR3_SPEC, IICSTAREQ_A, O>;
impl<'a, const O: u8> IICSTAREQ_W<'a, O> {
    #[doc = "Do not generate start condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICSTAREQ_A::_0)
    }
    #[doc = "Generate start condition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICSTAREQ_A::_1)
    }
}
#[doc = "Field `IICRSTAREQ` reader - Restart Condition Generation"]
pub type IICRSTAREQ_R = crate::BitReader<IICRSTAREQ_A>;
#[doc = "Restart Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRSTAREQ_A {
    #[doc = "0: Do not generate restart condition"]
    _0 = 0,
    #[doc = "1: Generate restart condition"]
    _1 = 1,
}
impl From<IICRSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICRSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICRSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICRSTAREQ_A {
        match self.bits {
            false => IICRSTAREQ_A::_0,
            true => IICRSTAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRSTAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRSTAREQ_A::_1
    }
}
#[doc = "Field `IICRSTAREQ` writer - Restart Condition Generation"]
pub type IICRSTAREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, SIMR3_SPEC, IICRSTAREQ_A, O>;
impl<'a, const O: u8> IICRSTAREQ_W<'a, O> {
    #[doc = "Do not generate restart condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICRSTAREQ_A::_0)
    }
    #[doc = "Generate restart condition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICRSTAREQ_A::_1)
    }
}
#[doc = "Field `IICSTPREQ` reader - Stop Condition Generation"]
pub type IICSTPREQ_R = crate::BitReader<IICSTPREQ_A>;
#[doc = "Stop Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTPREQ_A {
    #[doc = "0: Do not generate stop condition"]
    _0 = 0,
    #[doc = "1: Generate stop condition"]
    _1 = 1,
}
impl From<IICSTPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTPREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTPREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTPREQ_A {
        match self.bits {
            false => IICSTPREQ_A::_0,
            true => IICSTPREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTPREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTPREQ_A::_1
    }
}
#[doc = "Field `IICSTPREQ` writer - Stop Condition Generation"]
pub type IICSTPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, SIMR3_SPEC, IICSTPREQ_A, O>;
impl<'a, const O: u8> IICSTPREQ_W<'a, O> {
    #[doc = "Do not generate stop condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICSTPREQ_A::_0)
    }
    #[doc = "Generate stop condition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICSTPREQ_A::_1)
    }
}
#[doc = "Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag"]
pub type IICSTIF_R = crate::BitReader<IICSTIF_A>;
#[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTIF_A {
    #[doc = "0: No requests are being made for generating conditions, or a condition is being generated"]
    _0 = 0,
    #[doc = "1: Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0"]
    _1 = 1,
}
impl From<IICSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTIF_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTIF_A {
        match self.bits {
            false => IICSTIF_A::_0,
            true => IICSTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTIF_A::_1
    }
}
#[doc = "Field `IICSTIF` writer - Issuing of Start, Restart, or Stop Condition Completed Flag"]
pub type IICSTIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SIMR3_SPEC, IICSTIF_A, O>;
impl<'a, const O: u8> IICSTIF_W<'a, O> {
    #[doc = "No requests are being made for generating conditions, or a condition is being generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICSTIF_A::_0)
    }
    #[doc = "Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICSTIF_A::_1)
    }
}
#[doc = "Field `IICSDAS` reader - SDAn Output Select"]
pub type IICSDAS_R = crate::FieldReader<u8, IICSDAS_A>;
#[doc = "SDAn Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSDAS_A {
    #[doc = "0: Output serial data"]
    _00 = 0,
    #[doc = "1: Generate start, restart, or stop condition"]
    _01 = 1,
    #[doc = "2: Output low on SDAn pin"]
    _10 = 2,
    #[doc = "3: Drive SDAn pin to high-impedance state"]
    _11 = 3,
}
impl From<IICSDAS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSDAS_A) -> Self {
        variant as _
    }
}
impl IICSDAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSDAS_A {
        match self.bits {
            0 => IICSDAS_A::_00,
            1 => IICSDAS_A::_01,
            2 => IICSDAS_A::_10,
            3 => IICSDAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSDAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSDAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSDAS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSDAS_A::_11
    }
}
#[doc = "Field `IICSDAS` writer - SDAn Output Select"]
pub type IICSDAS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SIMR3_SPEC, u8, IICSDAS_A, 2, O>;
impl<'a, const O: u8> IICSDAS_W<'a, O> {
    #[doc = "Output serial data"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IICSDAS_A::_00)
    }
    #[doc = "Generate start, restart, or stop condition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IICSDAS_A::_01)
    }
    #[doc = "Output low on SDAn pin"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IICSDAS_A::_10)
    }
    #[doc = "Drive SDAn pin to high-impedance state"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IICSDAS_A::_11)
    }
}
#[doc = "Field `IICSCLS` reader - SCLn Output Select"]
pub type IICSCLS_R = crate::FieldReader<u8, IICSCLS_A>;
#[doc = "SCLn Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSCLS_A {
    #[doc = "0: Output serial clock"]
    _00 = 0,
    #[doc = "1: Generate start, restart, or stop condition"]
    _01 = 1,
    #[doc = "2: Output low on SCLn pin"]
    _10 = 2,
    #[doc = "3: Drive SCLn pin to high-impedance state"]
    _11 = 3,
}
impl From<IICSCLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSCLS_A) -> Self {
        variant as _
    }
}
impl IICSCLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSCLS_A {
        match self.bits {
            0 => IICSCLS_A::_00,
            1 => IICSCLS_A::_01,
            2 => IICSCLS_A::_10,
            3 => IICSCLS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSCLS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSCLS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSCLS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSCLS_A::_11
    }
}
#[doc = "Field `IICSCLS` writer - SCLn Output Select"]
pub type IICSCLS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SIMR3_SPEC, u8, IICSCLS_A, 2, O>;
impl<'a, const O: u8> IICSCLS_W<'a, O> {
    #[doc = "Output serial clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IICSCLS_A::_00)
    }
    #[doc = "Generate start, restart, or stop condition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IICSCLS_A::_01)
    }
    #[doc = "Output low on SCLn pin"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IICSCLS_A::_10)
    }
    #[doc = "Drive SCLn pin to high-impedance state"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IICSCLS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(&self) -> IICSTAREQ_R {
        IICSTAREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(&self) -> IICRSTAREQ_R {
        IICRSTAREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(&self) -> IICSTPREQ_R {
        IICSTPREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    pub fn iicstif(&self) -> IICSTIF_R {
        IICSTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SDAn Output Select"]
    #[inline(always)]
    pub fn iicsdas(&self) -> IICSDAS_R {
        IICSDAS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - SCLn Output Select"]
    #[inline(always)]
    pub fn iicscls(&self) -> IICSCLS_R {
        IICSCLS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstareq(&mut self) -> IICSTAREQ_W<0> {
        IICSTAREQ_W::new(self)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicrstareq(&mut self) -> IICRSTAREQ_W<1> {
        IICRSTAREQ_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstpreq(&mut self) -> IICSTPREQ_W<2> {
        IICSTPREQ_W::new(self)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    #[must_use]
    pub fn iicstif(&mut self) -> IICSTIF_W<3> {
        IICSTIF_W::new(self)
    }
    #[doc = "Bits 4:5 - SDAn Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicsdas(&mut self) -> IICSDAS_W<4> {
        IICSDAS_W::new(self)
    }
    #[doc = "Bits 6:7 - SCLn Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicscls(&mut self) -> IICSCLS_W<6> {
        IICSCLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IIC Mode Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr3](index.html) module"]
pub struct SIMR3_SPEC;
impl crate::RegisterSpec for SIMR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [simr3::R](R) reader structure"]
impl crate::Readable for SIMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [simr3::W](W) writer structure"]
impl crate::Writable for SIMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR3 to value 0"]
impl crate::Resettable for SIMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
