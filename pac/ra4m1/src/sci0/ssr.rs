#[doc = "Register `SSR` reader"]
pub struct R(crate::R<SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSR` writer"]
pub struct W(crate::W<SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_SPEC>;
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
impl From<crate::W<SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPBT` reader - Multi-Processor Bit Transfer"]
pub type MPBT_R = crate::BitReader<MPBT_A>;
#[doc = "Multi-Processor Bit Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_A {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPBT_A> for bool {
    #[inline(always)]
    fn from(variant: MPBT_A) -> Self {
        variant as u8 != 0
    }
}
impl MPBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPBT_A {
        match self.bits {
            false => MPBT_A::_0,
            true => MPBT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPBT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPBT_A::_1
    }
}
#[doc = "Field `MPBT` writer - Multi-Processor Bit Transfer"]
pub type MPBT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSR_SPEC, MPBT_A, O>;
impl<'a, const O: u8> MPBT_W<'a, O> {
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPBT_A::_0)
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPBT_A::_1)
    }
}
#[doc = "Field `MPB` reader - Multi-Processor"]
pub type MPB_R = crate::BitReader<MPB_A>;
#[doc = "Multi-Processor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
impl MPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::_0,
            true => MPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPB_A::_1
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag"]
pub type TEND_R = crate::BitReader<TEND_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    #[doc = "0: A character is being transmitted."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed."]
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
#[doc = "Field `PER` reader - Parity Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: A parity error has occurred"]
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
#[doc = "Field `PER` writer - Parity Error Flag"]
pub type PER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER_A::_0)
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER_A::_1)
    }
}
#[doc = "Field `FER` reader - Framing Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type FER_R = crate::BitReader<FER_A>;
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    #[doc = "0: No framing error occurred"]
    _0 = 0,
    #[doc = "1: A framing error has occurred"]
    _1 = 1,
}
impl From<FER_A> for bool {
    #[inline(always)]
    fn from(variant: FER_A) -> Self {
        variant as u8 != 0
    }
}
impl FER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FER_A {
        match self.bits {
            false => FER_A::_0,
            true => FER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FER_A::_1
    }
}
#[doc = "Field `FER` writer - Framing Error Flag"]
pub type FER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_SPEC, FER_A, O>;
impl<'a, const O: u8> FER_W<'a, O> {
    #[doc = "No framing error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FER_A::_0)
    }
    #[doc = "A framing error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FER_A::_1)
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: An overrun error has occurred"]
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
impl ORER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
#[doc = "Field `ORER` writer - Overrun Error Flag"]
pub type ORER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_SPEC, ORER_A, O>;
impl<'a, const O: u8> ORER_W<'a, O> {
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORER_A::_0)
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORER_A::_1)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Full Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: No received data is in RDR register"]
    _0 = 0,
    #[doc = "1: Received data is in RDR register"]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Field `RDRF` writer - Receive Data Full Flag"]
pub type RDRF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_SPEC, RDRF_A, O>;
impl<'a, const O: u8> RDRF_W<'a, O> {
    #[doc = "No received data is in RDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDRF_A::_0)
    }
    #[doc = "Received data is in RDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDRF_A::_1)
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: Transmit data is in TDR register"]
    _0 = 0,
    #[doc = "1: No transmit data is in TDR register"]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Field `TDRE` writer - Transmit Data Empty Flag"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, SSR_SPEC, TDRE_A, O>;
impl<'a, const O: u8> TDRE_W<'a, O> {
    #[doc = "Transmit data is in TDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_A::_0)
    }
    #[doc = "No transmit data is in TDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer"]
    #[inline(always)]
    pub fn mpbt(&self) -> MPBT_R {
        MPBT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multi-Processor"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<0> {
        MPBT_W::new(self)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<3> {
        PER_W::new(self)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fer(&mut self) -> FER_W<4> {
        FER_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn orer(&mut self) -> ORER_W<5> {
        ORER_W::new(self)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<6> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<7> {
        TDRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](index.html) module"]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssr::R](R) reader structure"]
impl crate::Readable for SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssr::W](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf8;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR to value 0x84"]
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x84;
}
