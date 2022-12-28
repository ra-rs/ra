#[doc = "Register `ADGSPCR` reader"]
pub struct R(crate::R<ADGSPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADGSPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADGSPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADGSPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADGSPCR` writer"]
pub struct W(crate::W<ADGSPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADGSPCR_SPEC>;
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
impl From<crate::W<ADGSPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADGSPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGS0` reader - ADC0 Group Priority Control Setting"]
pub type PGS0_R = crate::BitReader<PGS0_A>;
#[doc = "ADC0 Group Priority Control Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGS0_A {
    #[doc = "0: ADC0 operation is without group priority control"]
    _0 = 0,
    #[doc = "1: ADC0 operation is with group priority control in SAR mode. Setting prohibited when other than SAR mode."]
    _1 = 1,
}
impl From<PGS0_A> for bool {
    #[inline(always)]
    fn from(variant: PGS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PGS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGS0_A {
        match self.bits {
            false => PGS0_A::_0,
            true => PGS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGS0_A::_1
    }
}
#[doc = "Field `PGS0` writer - ADC0 Group Priority Control Setting"]
pub type PGS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, PGS0_A, O>;
impl<'a, const O: u8> PGS0_W<'a, O> {
    #[doc = "ADC0 operation is without group priority control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGS0_A::_0)
    }
    #[doc = "ADC0 operation is with group priority control in SAR mode. Setting prohibited when other than SAR mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGS0_A::_1)
    }
}
#[doc = "Field `RSCN0` reader - ADC0 Group Priority Control Setting 2"]
pub type RSCN0_R = crate::BitReader<RSCN0_A>;
#[doc = "ADC0 Group Priority Control Setting 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSCN0_A {
    #[doc = "0: Set when PGS0 is set to 0."]
    _0 = 0,
    #[doc = "1: Set when PGS0 is set to 1."]
    _1 = 1,
}
impl From<RSCN0_A> for bool {
    #[inline(always)]
    fn from(variant: RSCN0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSCN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSCN0_A {
        match self.bits {
            false => RSCN0_A::_0,
            true => RSCN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSCN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSCN0_A::_1
    }
}
#[doc = "Field `RSCN0` writer - ADC0 Group Priority Control Setting 2"]
pub type RSCN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, RSCN0_A, O>;
impl<'a, const O: u8> RSCN0_W<'a, O> {
    #[doc = "Set when PGS0 is set to 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSCN0_A::_0)
    }
    #[doc = "Set when PGS0 is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSCN0_A::_1)
    }
}
#[doc = "Field `LGRRS0` reader - ADC0 Group Priority Control Setting 3"]
pub type LGRRS0_R = crate::BitReader<LGRRS0_A>;
#[doc = "ADC0 Group Priority Control Setting 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LGRRS0_A {
    #[doc = "0: Set when PGS0 is set to 0."]
    _0 = 0,
    #[doc = "1: Set when PGS0 is set to 1."]
    _1 = 1,
}
impl From<LGRRS0_A> for bool {
    #[inline(always)]
    fn from(variant: LGRRS0_A) -> Self {
        variant as u8 != 0
    }
}
impl LGRRS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGRRS0_A {
        match self.bits {
            false => LGRRS0_A::_0,
            true => LGRRS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LGRRS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LGRRS0_A::_1
    }
}
#[doc = "Field `LGRRS0` writer - ADC0 Group Priority Control Setting 3"]
pub type LGRRS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, LGRRS0_A, O>;
impl<'a, const O: u8> LGRRS0_W<'a, O> {
    #[doc = "Set when PGS0 is set to 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGRRS0_A::_0)
    }
    #[doc = "Set when PGS0 is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGRRS0_A::_1)
    }
}
#[doc = "Field `GRP0` reader - ADC0 Group Priority Control Setting 4"]
pub type GRP0_R = crate::BitReader<GRP0_A>;
#[doc = "ADC0 Group Priority Control Setting 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRP0_A {
    #[doc = "0: Set the following case: When PGS0 is set to 0When PGS0 is set to 1 and ADC0 is SAR mode â\u{80}\u{93} Single scan mode."]
    _0 = 0,
    #[doc = "1: Set when PGS0 is set to 1 and ADC0 is SAR mode â\u{80}\u{93} Continuous scan mode."]
    _1 = 1,
}
impl From<GRP0_A> for bool {
    #[inline(always)]
    fn from(variant: GRP0_A) -> Self {
        variant as u8 != 0
    }
}
impl GRP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRP0_A {
        match self.bits {
            false => GRP0_A::_0,
            true => GRP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRP0_A::_1
    }
}
#[doc = "Field `GRP0` writer - ADC0 Group Priority Control Setting 4"]
pub type GRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, GRP0_A, O>;
impl<'a, const O: u8> GRP0_W<'a, O> {
    #[doc = "Set the following case: When PGS0 is set to 0When PGS0 is set to 1 and ADC0 is SAR mode â\u{80}\u{93} Single scan mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRP0_A::_0)
    }
    #[doc = "Set when PGS0 is set to 1 and ADC0 is SAR mode â\u{80}\u{93} Continuous scan mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRP0_A::_1)
    }
}
#[doc = "Field `PGS1` reader - ADC1 Group Priority Control Setting"]
pub type PGS1_R = crate::BitReader<PGS1_A>;
#[doc = "ADC1 Group Priority Control Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGS1_A {
    #[doc = "0: ADC1 operation is without group priority control"]
    _0 = 0,
    #[doc = "1: ADC1 operation is with group priority control in SAR mode. Setting prohibited when other than SAR mode."]
    _1 = 1,
}
impl From<PGS1_A> for bool {
    #[inline(always)]
    fn from(variant: PGS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PGS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGS1_A {
        match self.bits {
            false => PGS1_A::_0,
            true => PGS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGS1_A::_1
    }
}
#[doc = "Field `PGS1` writer - ADC1 Group Priority Control Setting"]
pub type PGS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, PGS1_A, O>;
impl<'a, const O: u8> PGS1_W<'a, O> {
    #[doc = "ADC1 operation is without group priority control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGS1_A::_0)
    }
    #[doc = "ADC1 operation is with group priority control in SAR mode. Setting prohibited when other than SAR mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGS1_A::_1)
    }
}
#[doc = "Field `RSCN1` reader - ADC1 Group Priority Control Setting 2"]
pub type RSCN1_R = crate::BitReader<RSCN1_A>;
#[doc = "ADC1 Group Priority Control Setting 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSCN1_A {
    #[doc = "0: Set when PGS1 is set to 0."]
    _0 = 0,
    #[doc = "1: Set when PGS1 is set to 1."]
    _1 = 1,
}
impl From<RSCN1_A> for bool {
    #[inline(always)]
    fn from(variant: RSCN1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSCN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSCN1_A {
        match self.bits {
            false => RSCN1_A::_0,
            true => RSCN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSCN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSCN1_A::_1
    }
}
#[doc = "Field `RSCN1` writer - ADC1 Group Priority Control Setting 2"]
pub type RSCN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, RSCN1_A, O>;
impl<'a, const O: u8> RSCN1_W<'a, O> {
    #[doc = "Set when PGS1 is set to 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSCN1_A::_0)
    }
    #[doc = "Set when PGS1 is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSCN1_A::_1)
    }
}
#[doc = "Field `LGRRS1` reader - ADC1 Group Priority Control Setting 3"]
pub type LGRRS1_R = crate::BitReader<LGRRS1_A>;
#[doc = "ADC1 Group Priority Control Setting 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LGRRS1_A {
    #[doc = "0: Set when PGS1 is set to 0."]
    _0 = 0,
    #[doc = "1: Set when PGS1 is set to 1."]
    _1 = 1,
}
impl From<LGRRS1_A> for bool {
    #[inline(always)]
    fn from(variant: LGRRS1_A) -> Self {
        variant as u8 != 0
    }
}
impl LGRRS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGRRS1_A {
        match self.bits {
            false => LGRRS1_A::_0,
            true => LGRRS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LGRRS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LGRRS1_A::_1
    }
}
#[doc = "Field `LGRRS1` writer - ADC1 Group Priority Control Setting 3"]
pub type LGRRS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, LGRRS1_A, O>;
impl<'a, const O: u8> LGRRS1_W<'a, O> {
    #[doc = "Set when PGS1 is set to 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGRRS1_A::_0)
    }
    #[doc = "Set when PGS1 is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGRRS1_A::_1)
    }
}
#[doc = "Field `GRP1` reader - ADC1 Group Priority Control Setting 4"]
pub type GRP1_R = crate::BitReader<GRP1_A>;
#[doc = "ADC1 Group Priority Control Setting 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRP1_A {
    #[doc = "0: Set the following case: When PGS1 is set to 0When PGS1 is set to 1 and ADC1 is SAR mode â\u{80}\u{93} Single scan mode."]
    _0 = 0,
    #[doc = "1: Set when PGS1 is set to 1 and ADC1 is SAR mode â\u{80}\u{93} Continuous scan mode."]
    _1 = 1,
}
impl From<GRP1_A> for bool {
    #[inline(always)]
    fn from(variant: GRP1_A) -> Self {
        variant as u8 != 0
    }
}
impl GRP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRP1_A {
        match self.bits {
            false => GRP1_A::_0,
            true => GRP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRP1_A::_1
    }
}
#[doc = "Field `GRP1` writer - ADC1 Group Priority Control Setting 4"]
pub type GRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADGSPCR_SPEC, GRP1_A, O>;
impl<'a, const O: u8> GRP1_W<'a, O> {
    #[doc = "Set the following case: When PGS1 is set to 0When PGS1 is set to 1 and ADC1 is SAR mode â\u{80}\u{93} Single scan mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRP1_A::_0)
    }
    #[doc = "Set when PGS1 is set to 1 and ADC1 is SAR mode â\u{80}\u{93} Continuous scan mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRP1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 Group Priority Control Setting"]
    #[inline(always)]
    pub fn pgs0(&self) -> PGS0_R {
        PGS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 Group Priority Control Setting 2"]
    #[inline(always)]
    pub fn rscn0(&self) -> RSCN0_R {
        RSCN0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC0 Group Priority Control Setting 3"]
    #[inline(always)]
    pub fn lgrrs0(&self) -> LGRRS0_R {
        LGRRS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC0 Group Priority Control Setting 4"]
    #[inline(always)]
    pub fn grp0(&self) -> GRP0_R {
        GRP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 Group Priority Control Setting"]
    #[inline(always)]
    pub fn pgs1(&self) -> PGS1_R {
        PGS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 Group Priority Control Setting 2"]
    #[inline(always)]
    pub fn rscn1(&self) -> RSCN1_R {
        RSCN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 Group Priority Control Setting 3"]
    #[inline(always)]
    pub fn lgrrs1(&self) -> LGRRS1_R {
        LGRRS1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC1 Group Priority Control Setting 4"]
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 Group Priority Control Setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgs0(&mut self) -> PGS0_W<0> {
        PGS0_W::new(self)
    }
    #[doc = "Bit 1 - ADC0 Group Priority Control Setting 2"]
    #[inline(always)]
    #[must_use]
    pub fn rscn0(&mut self) -> RSCN0_W<1> {
        RSCN0_W::new(self)
    }
    #[doc = "Bit 2 - ADC0 Group Priority Control Setting 3"]
    #[inline(always)]
    #[must_use]
    pub fn lgrrs0(&mut self) -> LGRRS0_W<2> {
        LGRRS0_W::new(self)
    }
    #[doc = "Bit 3 - ADC0 Group Priority Control Setting 4"]
    #[inline(always)]
    #[must_use]
    pub fn grp0(&mut self) -> GRP0_W<3> {
        GRP0_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 Group Priority Control Setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgs1(&mut self) -> PGS1_W<8> {
        PGS1_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 Group Priority Control Setting 2"]
    #[inline(always)]
    #[must_use]
    pub fn rscn1(&mut self) -> RSCN1_W<9> {
        RSCN1_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 Group Priority Control Setting 3"]
    #[inline(always)]
    #[must_use]
    pub fn lgrrs1(&mut self) -> LGRRS1_W<10> {
        LGRRS1_W::new(self)
    }
    #[doc = "Bit 11 - ADC1 Group Priority Control Setting 4"]
    #[inline(always)]
    #[must_use]
    pub fn grp1(&mut self) -> GRP1_W<11> {
        GRP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Group scan Priority Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adgspcr](index.html) module"]
pub struct ADGSPCR_SPEC;
impl crate::RegisterSpec for ADGSPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adgspcr::R](R) reader structure"]
impl crate::Readable for ADGSPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adgspcr::W](W) writer structure"]
impl crate::Writable for ADGSPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADGSPCR to value 0"]
impl crate::Resettable for ADGSPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
