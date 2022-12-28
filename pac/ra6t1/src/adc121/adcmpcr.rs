#[doc = "Register `ADCMPCR` reader"]
pub struct R(crate::R<ADCMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPCR` writer"]
pub struct W(crate::W<ADCMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPCR_SPEC>;
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
impl From<crate::W<ADCMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPAB` reader - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
pub type CMPAB_R = crate::FieldReader<u8, CMPAB_A>;
#[doc = "Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPAB_A {
    #[doc = "0: S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    _00 = 0,
    #[doc = "1: S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    _01 = 1,
    #[doc = "2: S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<CMPAB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPAB_A) -> Self {
        variant as _
    }
}
impl CMPAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPAB_A {
        match self.bits {
            0 => CMPAB_A::_00,
            1 => CMPAB_A::_01,
            2 => CMPAB_A::_10,
            3 => CMPAB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPAB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPAB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPAB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPAB_A::_11
    }
}
#[doc = "Field `CMPAB` writer - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
pub type CMPAB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCMPCR_SPEC, u8, CMPAB_A, 2, O>;
impl<'a, const O: u8> CMPAB_W<'a, O> {
    #[doc = "S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMPAB_A::_00)
    }
    #[doc = "S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMPAB_A::_01)
    }
    #[doc = "S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPAB_A::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPAB_A::_11)
    }
}
#[doc = "Field `CMPBE` reader - Compare Window B Operation Enable"]
pub type CMPBE_R = crate::BitReader<CMPBE_A>;
#[doc = "Compare Window B Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPBE_A {
    #[doc = "0: Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
    _0 = 0,
    #[doc = "1: Compare window B operation is enabled."]
    _1 = 1,
}
impl From<CMPBE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPBE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPBE_A {
        match self.bits {
            false => CMPBE_A::_0,
            true => CMPBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPBE_A::_1
    }
}
#[doc = "Field `CMPBE` writer - Compare Window B Operation Enable"]
pub type CMPBE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPCR_SPEC, CMPBE_A, O>;
impl<'a, const O: u8> CMPBE_W<'a, O> {
    #[doc = "Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPBE_A::_0)
    }
    #[doc = "Compare window B operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPBE_A::_1)
    }
}
#[doc = "Field `CMPAE` reader - Compare Window A Operation Enable"]
pub type CMPAE_R = crate::BitReader<CMPAE_A>;
#[doc = "Compare Window A Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPAE_A {
    #[doc = "0: Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
    _0 = 0,
    #[doc = "1: Compare window A operation is enabled."]
    _1 = 1,
}
impl From<CMPAE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPAE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPAE_A {
        match self.bits {
            false => CMPAE_A::_0,
            true => CMPAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPAE_A::_1
    }
}
#[doc = "Field `CMPAE` writer - Compare Window A Operation Enable"]
pub type CMPAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPCR_SPEC, CMPAE_A, O>;
impl<'a, const O: u8> CMPAE_W<'a, O> {
    #[doc = "Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPAE_A::_0)
    }
    #[doc = "Compare window A operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPAE_A::_1)
    }
}
#[doc = "Field `CMPBIE` reader - Compare B Interrupt Enable"]
pub type CMPBIE_R = crate::BitReader<CMPBIE_A>;
#[doc = "Compare B Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPBIE_A {
    #[doc = "0: S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met."]
    _0 = 0,
    #[doc = "1: S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met."]
    _1 = 1,
}
impl From<CMPBIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPBIE_A {
        match self.bits {
            false => CMPBIE_A::_0,
            true => CMPBIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPBIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPBIE_A::_1
    }
}
#[doc = "Field `CMPBIE` writer - Compare B Interrupt Enable"]
pub type CMPBIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPCR_SPEC, CMPBIE_A, O>;
impl<'a, const O: u8> CMPBIE_W<'a, O> {
    #[doc = "S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPBIE_A::_0)
    }
    #[doc = "S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPBIE_A::_1)
    }
}
#[doc = "Field `WCMPE` reader - Window Function Setting"]
pub type WCMPE_R = crate::BitReader<WCMPE_A>;
#[doc = "Window Function Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCMPE_A {
    #[doc = "0: Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    _0 = 0,
    #[doc = "1: Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    _1 = 1,
}
impl From<WCMPE_A> for bool {
    #[inline(always)]
    fn from(variant: WCMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl WCMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCMPE_A {
        match self.bits {
            false => WCMPE_A::_0,
            true => WCMPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WCMPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WCMPE_A::_1
    }
}
#[doc = "Field `WCMPE` writer - Window Function Setting"]
pub type WCMPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPCR_SPEC, WCMPE_A, O>;
impl<'a, const O: u8> WCMPE_W<'a, O> {
    #[doc = "Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WCMPE_A::_0)
    }
    #[doc = "Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WCMPE_A::_1)
    }
}
#[doc = "Field `CMPAIE` reader - Compare A Interrupt Enable"]
pub type CMPAIE_R = crate::BitReader<CMPAIE_A>;
#[doc = "Compare A Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPAIE_A {
    #[doc = "0: S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met."]
    _0 = 0,
    #[doc = "1: S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met."]
    _1 = 1,
}
impl From<CMPAIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPAIE_A {
        match self.bits {
            false => CMPAIE_A::_0,
            true => CMPAIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPAIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPAIE_A::_1
    }
}
#[doc = "Field `CMPAIE` writer - Compare A Interrupt Enable"]
pub type CMPAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCMPCR_SPEC, CMPAIE_A, O>;
impl<'a, const O: u8> CMPAIE_W<'a, O> {
    #[doc = "S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPAIE_A::_0)
    }
    #[doc = "S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPAIE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    pub fn cmpab(&self) -> CMPAB_R {
        CMPAB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 9 - Compare Window B Operation Enable"]
    #[inline(always)]
    pub fn cmpbe(&self) -> CMPBE_R {
        CMPBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Window A Operation Enable"]
    #[inline(always)]
    pub fn cmpae(&self) -> CMPAE_R {
        CMPAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare B Interrupt Enable"]
    #[inline(always)]
    pub fn cmpbie(&self) -> CMPBIE_R {
        CMPBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Window Function Setting"]
    #[inline(always)]
    pub fn wcmpe(&self) -> WCMPE_R {
        WCMPE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare A Interrupt Enable"]
    #[inline(always)]
    pub fn cmpaie(&self) -> CMPAIE_R {
        CMPAIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn cmpab(&mut self) -> CMPAB_W<0> {
        CMPAB_W::new(self)
    }
    #[doc = "Bit 9 - Compare Window B Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbe(&mut self) -> CMPBE_W<9> {
        CMPBE_W::new(self)
    }
    #[doc = "Bit 11 - Compare Window A Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpae(&mut self) -> CMPAE_W<11> {
        CMPAE_W::new(self)
    }
    #[doc = "Bit 13 - Compare B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbie(&mut self) -> CMPBIE_W<13> {
        CMPBIE_W::new(self)
    }
    #[doc = "Bit 14 - Window Function Setting"]
    #[inline(always)]
    #[must_use]
    pub fn wcmpe(&mut self) -> WCMPE_W<14> {
        WCMPE_W::new(self)
    }
    #[doc = "Bit 15 - Compare A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaie(&mut self) -> CMPAIE_W<15> {
        CMPAIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpcr](index.html) module"]
pub struct ADCMPCR_SPEC;
impl crate::RegisterSpec for ADCMPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpcr::R](R) reader structure"]
impl crate::Readable for ADCMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpcr::W](W) writer structure"]
impl crate::Writable for ADCMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPCR to value 0"]
impl crate::Resettable for ADCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
